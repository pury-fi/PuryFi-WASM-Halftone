#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]

use std::{cmp, f64};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

fn sample_channel(
    data: &[u8],
    w: usize,
    h: usize,
    xc: f64,
    yc: f64,
    size: f64,
    channel: usize,
    is_key: bool,
    accurate_sampling: bool,
) -> f64 {
    if accurate_sampling {
        let start_i = cmp::max(0, (xc - size / 2.0).floor() as usize);
        let start_j = cmp::max(0, (yc - size / 2.0).floor() as usize);
        let end_i = cmp::min(w, (xc + size / 2.0).ceil() as usize);
        let end_j = cmp::min(h, (yc + size / 2.0).ceil() as usize);

        let mut val: i64 = 0;
        for i in start_i..end_i {
            for j in start_j..end_j {
                let index = (i + j * w) * 4;

                let key_val =
                    255 - cmp::max(data[index], cmp::max(data[index + 1], data[index + 2])) as i64;
                if is_key {
                    val += key_val;
                } else {
                    val += 255 - data[index + channel] as i64 - key_val;
                }
            }
        }

        let count = (end_i - start_i) * (end_j - start_j);
        val as f64 / count as f64
    } else {
        let index = (xc as usize + yc as usize * w) * 4;

        let key_val =
            255 - cmp::max(data[index], cmp::max(data[index + 1], data[index + 2])) as i64;
        if is_key {
            key_val as f64
        } else {
            (255 - data[index + channel] as i64 - key_val) as f64
        }
    }
}

fn sample_channel_average(
    data: &[u8],
    w: usize,
    h: usize,
    xc: f64,
    yc: f64,
    size: f64,
    accurate_sampling: bool,
) -> f64 {
    if accurate_sampling {
        let start_i = cmp::max(0, (xc - size / 2.0).floor() as usize);
        let start_j = cmp::max(0, (yc - size / 2.0).floor() as usize);
        let end_i = cmp::min(w, (xc + size / 2.0).ceil() as usize);
        let end_j = cmp::min(h, (yc + size / 2.0).ceil() as usize);

        let mut val = 0;
        for i in start_i..end_i {
            for j in start_j..end_j {
                let index = (i + j * w) * 4;
                val += data[index] as usize + data[index + 1] as usize + data[index + 2] as usize;
            }
        }

        let count = (end_i - start_i) * (end_j - start_j);
        255.0 - (val as f64 / count as f64 / 3.0)
    } else {
        let index = (xc as usize + yc as usize * w) * 4;
        255.0 - ((data[index] as f64 + data[index + 1] as f64 + data[index + 2] as f64) / 3.0)
    }
}

fn add_fill_circle_path(path_data: &mut [f64], w: usize, h: usize, x: f64, y: f64, radius: f64) {
    let x = x - 0.5;
    let y = y - 0.5;

    let base_alpha = if radius < 0.5 { radius * 2.0 } else { 1.0 };

    let start_x = cmp::max(0, (x - radius - 1.0).floor() as usize) as usize;
    let start_y = cmp::max(0, (y - radius - 1.0).floor() as usize) as usize;
    let end_x = cmp::min(w, (x + radius + 1.0).ceil() as usize) as usize;
    let end_y = cmp::min(h, (y + radius + 1.0).ceil() as usize) as usize;

    let radius_sq = radius.powi(2);

    for j in start_y..end_y {
        for i in start_x..end_x {
            let dist = ((i as f64 - x).powi(2) + (j as f64 - y).powi(2)).sqrt();
            if dist > radius + 1.0 {
                continue;
            }

            let a = (1.0 - (dist - radius + 0.5).max(0.0)) * base_alpha;

            let index = i + j * w;
            path_data[index] = a.max(path_data[index]);
        }
    }
}

fn paint_path(
    data: &mut [u8],
    path_data: &[f64],
    w: usize,
    h: usize,
    r: u8,
    g: u8,
    b: u8,
    alpha: f64,
) {
    for j in 0..h {
        for i in 0..w {
            let a = path_data[i + j * w] * alpha;
            let inv_a = 1.0 - a;

            let index = (i + j * w) * 4;

            data[index] = (r as f64 * a + data[index] as f64 * inv_a) as u8;
            data[index + 1] = (g as f64 * a + data[index + 1] as f64 * inv_a) as u8;
            data[index + 2] = (b as f64 * a + data[index + 2] as f64 * inv_a) as u8;
        }
    }
}

fn paint_fill(data: &mut [u8],
    w: usize,
    h: usize,
    r: u8,
    g: u8,
    b: u8,
    alpha: f64) {
    for j in 0..h {
        for i in 0..w {
            let index = (i + j * w) * 4;
            let inv_a = 1.0 - alpha;

            data[index] = (r as f64 * alpha + data[index] as f64 * inv_a) as u8;
            data[index + 1] =
                (g as f64 * alpha + data[index + 1] as f64 * inv_a) as u8;
            data[index + 2] =
                (b as f64 * alpha + data[index + 2] as f64 * inv_a) as u8;
        }
    }
}

fn paint_darken_fill_circle(
    data: &mut [u8],
    w: usize,
    h: usize,
    r: u8,
    g: u8,
    b: u8,
    alpha: f64,
    x: f64,
    y: f64,
    radius: f64,
) {
    let x = x - 0.5;
    let y = y - 0.5;

    let base_alpha = if radius < 0.5 { radius * 2.0 } else { 1.0 };

    let start_x = cmp::max(0, (x - radius - 1.0).floor() as usize) as usize;
    let start_y = cmp::max(0, (y - radius - 1.0).floor() as usize) as usize;
    let end_x = cmp::min(w, (x + radius + 1.0).ceil() as usize) as usize;
    let end_y = cmp::min(h, (y + radius + 1.0).ceil() as usize) as usize;

    for j in start_y..end_y {
        for i in start_x..end_x {
            let dist = ((i as f64 - x).powi(2) + (j as f64 - y).powi(2)).sqrt();
            if dist > radius + 1.0 {
                continue;
            }

            let a = (1.0 - (dist - radius + 0.5).max(0.0)) * base_alpha;

            let index = (i + j * w) * 4;
            let inv_a = 1.0 - a;

            data[index] = ((r as f64 * a + data[index] as f64 * inv_a) as u8).min(data[index]);
            data[index + 1] =
                ((g as f64 * a + data[index + 1] as f64 * inv_a) as u8).min(data[index + 1]);
            data[index + 2] =
                ((b as f64 * a + data[index + 2] as f64 * inv_a) as u8).min(data[index + 2]);
        }
    }
}

fn for_each_rotated_grid_point_in_rect(
    data_x: f64,
    data_y: f64,
    data_w: f64,
    data_h: f64,
    rect_w: f64,
    rect_h: f64,
    rect_x: f64,
    rect_y: f64,
    angle: f64,
    dot_res: f64,
    action: &mut dyn FnMut(f64, f64),
) {
    let radians = (angle * f64::consts::PI) / 180.0;

    let cos = radians.cos();
    let sin = radians.sin();

    let tl = [rect_x, rect_y];
    let tr = [rect_x + rect_w, rect_y];
    let br = [rect_x + rect_w, rect_y + rect_h];
    let bl = [rect_x, rect_y + rect_h];
    let boundaries = [tl, br, tr, bl]
        .iter()
        .map(|[x, y]| {
            [
                (x - rect_w / 2.0) * cos - (y - rect_h / 2.0) * sin + rect_w / 2.0,
                (x - rect_w / 2.0) * sin + (y - rect_h / 2.0) * cos + rect_h / 2.0,
            ]
        })
        .collect::<Vec<[f64; 2]>>();

    let min_x = boundaries
        .iter()
        .map(|point| point[0])
        .fold(f64::INFINITY, f64::min);
    let min_y = boundaries
        .iter()
        .map(|point| point[1])
        .fold(f64::INFINITY, f64::min);
    let max_y = boundaries
        .iter()
        .map(|point| point[1])
        .fold(f64::NEG_INFINITY, f64::max);
    let max_x = boundaries
        .iter()
        .map(|point| point[0])
        .fold(f64::NEG_INFINITY, f64::max);

    let inv_cos = (-radians).cos();
    let inv_sin = (-radians).sin();

    let x_step = dot_res * inv_cos;
    let y_step = dot_res * inv_sin;

    let base_rot_x = (min_x - rect_w / 2.0) * inv_cos + rect_w / 2.0;
    let base_rot_y = (min_x - rect_w / 2.0) * inv_sin + rect_h / 2.0;
    let mut y = min_y - rect_h / 2.0;
    while y < max_y - rect_h / 2.0 {
        let mut rot_x = base_rot_x - y * inv_sin;
        let mut rot_y = base_rot_y + y * inv_cos;
        while rot_x < data_x + data_w {
            if rot_x < data_x || rot_y < data_y || rot_y > data_y + data_h {
                rot_x += x_step;
                rot_y += y_step;
                continue;
            }

            action(rot_x, rot_y);

            rot_x += x_step;
            rot_y += y_step;
        }
        y += dot_res;
    }
}

#[wasm_bindgen]
pub fn halftone_pixel(
    ctx: &CanvasRenderingContext2d,
    src_ctx: &CanvasRenderingContext2d,
    rect_x: f64,
    rect_y: f64,
    rect_w: f64,
    rect_h: f64,
    px_size: f64,
    density: f64,
    r: u8,
    g: u8,
    b: u8,
    alpha: f64,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    bg_alpha: f64,
    accurate_sampling: bool,
    cmyk_color_mode: bool,
) {
    let dot_size = (px_size.powf(2.0) / f64::consts::PI).sqrt();
    let dot_res = (dot_size / density).max(1.0).round();

    let canvas_w = ctx.canvas().unwrap().width() as i32;
    let canvas_h = ctx.canvas().unwrap().height() as i32;

    let data_x = (rect_x - dot_size).floor().max(0.0);
    let data_y = (rect_y - dot_size).floor().max(0.0);
    let data_w =
        ((rect_x + rect_w + dot_size).min(canvas_w as f64 + dot_size) - rect_x + dot_size).ceil();
    let data_h =
        ((rect_y + rect_h + dot_size).min(canvas_h as f64 + dot_size) - rect_y + dot_size).ceil();

    let mut src_data = src_ctx
        .get_image_data(data_x, data_y, data_w, data_h)
        .unwrap()
        .data()
        .to_vec();

    if cmyk_color_mode {
        let mut data = vec![255_u8; (data_w * data_h * 4.0) as usize];
        for (angle, (r, g, b), channel, is_key) in [
            (2.0, (255, 255, 0), 2, false),
            (75.0, (255, 0, 255), 1, false),
            (15.0, (0, 255, 255), 0, false),
            (45.0, (0, 0, 0), 0, true),
        ] {
            for_each_rotated_grid_point_in_rect(
                data_x,
                data_y,
                data_w,
                data_h,
                rect_w,
                rect_h,
                rect_x,
                rect_y,
                angle,
                dot_res,
                &mut |rot_x, rot_y| {
                    let val = sample_channel(
                        &src_data,
                        data_w as usize,
                        data_h as usize,
                        rot_x - data_x,
                        rot_y - data_y,
                        dot_size,
                        channel,
                        is_key,
                        accurate_sampling,
                    );

                    let circle_r = (val / 255.0) * dot_size / 2.0;
                    paint_darken_fill_circle(
                        &mut data,
                        data_w as usize,
                        data_h as usize,
                        r,
                        g,
                        b,
                        alpha,
                        rot_x - data_x,
                        rot_y - data_y,
                        circle_r,
                    );
                },
            );
        }

        let out_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            data_w as u32,
            data_h as u32,
        )
        .unwrap();
        ctx.put_image_data(&out_data, data_x, data_y).unwrap();
    } else {
        let mut path_data = vec![0_f64; data_w as usize * data_h as usize];
        for_each_rotated_grid_point_in_rect(
            data_x,
            data_y,
            data_w,
            data_h,
            rect_w,
            rect_h,
            rect_x,
            rect_y,
            45.0,
            dot_res,
            &mut |rot_x, rot_y| {
                let val = sample_channel_average(
                    &src_data,
                    data_w as usize,
                    data_h as usize,
                    rot_x - data_x,
                    rot_y - data_y,
                    dot_size,
                    accurate_sampling,
                );

                let circle_r = (val / 255.0) * dot_size / 2.0;
                add_fill_circle_path(
                    &mut path_data,
                    data_w as usize,
                    data_h as usize,
                    rot_x - data_x,
                    rot_y - data_y,
                    circle_r,
                );
            },
        );

        paint_fill(&mut src_data, data_w as usize, data_h as usize, bg_r, bg_g, bg_b, bg_alpha);
        paint_path(
            &mut src_data,
            &path_data,
            data_w as usize,
            data_h as usize,
            r,
            g,
            b,
            alpha,
        );

        let out_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&src_data),
            data_w as u32,
            data_h as u32,
        )
        .unwrap();
        ctx.put_image_data(&out_data, data_x, data_y).unwrap();
    }
}
