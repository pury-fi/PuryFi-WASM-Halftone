/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} src_data
* @param {number} dot_size
* @param {number} dot_res
* @param {number} data_x
* @param {number} data_y
* @param {number} data_w
* @param {number} data_h
* @param {number} rect_x
* @param {number} rect_y
* @param {number} rect_w
* @param {number} rect_h
* @param {number} r
* @param {number} g
* @param {number} b
* @param {number} alpha
* @param {number} bg_r
* @param {number} bg_g
* @param {number} bg_b
* @param {number} bg_alpha
* @param {boolean} accurate_sampling
* @param {boolean} cmyk_color_mode
* @returns {Uint8Array}
*/
export function halftone_pixel(src_data: Uint8Array, dot_size: number, dot_res: number, data_x: number, data_y: number, data_w: number, data_h: number, rect_x: number, rect_y: number, rect_w: number, rect_h: number, r: number, g: number, b: number, alpha: number, bg_r: number, bg_g: number, bg_b: number, bg_alpha: number, accurate_sampling: boolean, cmyk_color_mode: boolean): Uint8Array;
