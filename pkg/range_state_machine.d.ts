/* tslint:disable */
/* eslint-disable */
/**
*/
export function setup_for_debug(): void;
/**
*/
export class RangeCanvas {
  free(): void;
/**
*/
  constructor();
/**
* @param {number} x 
* @param {number} y 
*/
  on_mouse_down(x: number, y: number): void;
/**
* @param {number} x 
* @param {number} y 
*/
  on_mouse_up(x: number, y: number): void;
/**
* @param {number} x 
* @param {number} y 
*/
  on_mouse_move(x: number, y: number): void;
/**
* @param {string} key 
*/
  on_key_up(key: string): void;
}

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        