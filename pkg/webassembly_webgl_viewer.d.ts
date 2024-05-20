/* tslint:disable */
/* eslint-disable */
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
* @returns {Float32Array}
*/
export function update_sides(n: number): Float32Array;
/**
* @param {string} canvas_id
* @param {number} i
* @returns {WebGLRenderingContext}
*/
export function update_colors(canvas_id: string, i: number): WebGLRenderingContext;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly draw_triangle: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly draw_square: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly update_sides: (a: number, b: number) => void;
  readonly update_colors: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
