/* tslint:disable */
/* eslint-disable */
/**
* @param {number} res
*/
export function s_update_resolution(res: number): void;
/**
* @param {number} w
*/
export function s_update_wavelength(w: number): void;
/**
* @param {number} x
* @param {number} y
*/
export function s_mouse_move(x: number, y: number): void;
/**
* @param {string} canvas_id
* @returns {WebGLRenderingContext}
*/
export function init_gl(canvas_id: string): WebGLRenderingContext;
/**
* @param {number} x
* @param {number} y
*/
export function mouse_move(x: number, y: number): void;
/**
* @param {string} c_id
*/
export function sin_draw(c_id: string): void;
/**
* @param {number} n
*/
export function update_sides(n: number): void;
/**
* @param {number} s
*/
export function update_rotation_speed(s: number): void;
/**
* @param {number} s
*/
export function update_colour_speed(s: number): void;
/**
* @param {string} canvas_id
* @returns {WebGLRenderingContext}
*/
export function draw(canvas_id: string): WebGLRenderingContext;
/**
*/
export function user_init(): void;
/**
* @param {string} canvas_id
* @param {Float32Array | undefined} [selected_color]
* @returns {WebGLRenderingContext}
*/
export function draw_triangle(canvas_id: string, selected_color?: Float32Array): WebGLRenderingContext;
/**
* @param {number} n
*/
export function g_update_sides(n: number): void;
/**
* @param {string} canvas_id
* @returns {WebGLRenderingContext}
*/
export function gradient_draw(canvas_id: string): WebGLRenderingContext;
/**
* @param {string} canvas_id
* @param {Float32Array | undefined} [selected_color]
* @returns {WebGLRenderingContext}
*/
export function draw_square(canvas_id: string, selected_color?: Float32Array): WebGLRenderingContext;
/**
* @param {boolean} checked
*/
export function p_update_box(checked: boolean): void;
/**
* @param {number} res
*/
export function p_update_resolution(res: number): void;
/**
* @param {string} canvas_id
* @returns {WebGLRenderingContext}
*/
export function point_draw(canvas_id: string): WebGLRenderingContext;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly s_update_resolution: (a: number) => void;
  readonly s_update_wavelength: (a: number) => void;
  readonly s_mouse_move: (a: number, b: number) => void;
  readonly init_gl: (a: number, b: number) => number;
  readonly mouse_move: (a: number, b: number) => void;
  readonly sin_draw: (a: number, b: number) => void;
  readonly update_sides: (a: number) => void;
  readonly update_rotation_speed: (a: number) => void;
  readonly update_colour_speed: (a: number) => void;
  readonly draw: (a: number, b: number, c: number) => void;
  readonly user_init: () => void;
  readonly draw_triangle: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly g_update_sides: (a: number) => void;
  readonly gradient_draw: (a: number, b: number, c: number) => void;
  readonly draw_square: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly p_update_box: (a: number) => void;
  readonly p_update_resolution: (a: number) => void;
  readonly point_draw: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf3bb657d9ab02cf6: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hefa01079700a9b33: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
