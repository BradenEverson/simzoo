/* tslint:disable */
/* eslint-disable */
/**
 * Conway's Game of Life
 */
export class Conway {
  private constructor();
  free(): void;
  /**
   * Creates a new simulation with set dimensions
   */
  static with_dims(width: number, height: number): Conway;
  get_width(): number;
  get_height(): number;
  resize(width: number, height: number): void;
  step(): void;
  set(idx: number): void;
  render(): Uint32Array;
  steps(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_conway_free: (a: number, b: number) => void;
  readonly conway_with_dims: (a: number, b: number) => number;
  readonly conway_get_width: (a: number) => number;
  readonly conway_get_height: (a: number) => number;
  readonly conway_resize: (a: number, b: number, c: number) => void;
  readonly conway_step: (a: number) => void;
  readonly conway_set: (a: number, b: number) => void;
  readonly conway_render: (a: number) => [number, number];
  readonly conway_steps: (a: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
