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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly halftone_pixel: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
