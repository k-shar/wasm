/* tslint:disable */
/* eslint-disable */
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
* @param {string} canvas_id
* @param {Float32Array | undefined} [selected_color]
* @returns {WebGLRenderingContext}
*/
export function draw_triangle(canvas_id: string, selected_color?: Float32Array): WebGLRenderingContext;
/**
* @param {string} canvas_id
* @param {Float32Array | undefined} [selected_color]
* @returns {WebGLRenderingContext}
*/
export function draw_square(canvas_id: string, selected_color?: Float32Array): WebGLRenderingContext;
/**
* @param {number} n
*/
export function g_update_sides(n: number): void;
/**
* @param {string} canvas_id
* @returns {WebGLRenderingContext}
*/
export function gradient_draw(canvas_id: string): WebGLRenderingContext;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly update_sides: (a: number) => void;
  readonly update_rotation_speed: (a: number) => void;
  readonly update_colour_speed: (a: number) => void;
  readonly draw: (a: number, b: number, c: number) => void;
  readonly draw_triangle: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly draw_square: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly g_update_sides: (a: number) => void;
  readonly gradient_draw: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
