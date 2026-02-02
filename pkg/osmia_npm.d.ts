/* tslint:disable */
/* eslint-disable */

/**
 * Get the Osmia context, based on a given initial context.
 *
 * # Arguments
 * * `ctx` - The initial context as a string
 *
 * # Returns
 * * `Result<String, String>` - The Osmia context as a string, or an error message
 */
export function ctx_json_dump(ctx?: string | null): string;

/**
 * Get a variable from the Osmia context, based on a given initial context.
 *
 * # Arguments
 * * `var` - The name of the variable to get
 * * `ctx` - The initial context as a string
 *
 * # Returns
 * * `Result<String, String>` - The value of the variable, or an error message
 */
export function ctx_json_dump_variable(_var: string, ctx?: string | null): string;

/**
 * Run Osmia code with the default context
 *
 * # Arguments
 * * `code` - The Osmia code to run as a string
 *
 * # Returns
 * * `Result<String, String>` - The result of the Osmia code, or an error message
 */
export function run(code: string): string;

/**
 * Run Osmia code with a custom context
 *
 * # Arguments
 * * `ctx` - The Osmia context as a string (JSON)
 * * `code` - The Osmia code to run as a string
 *
 * # Returns
 * * `Result<String, String>` - The result of the Osmia code, or an error message
 */
export function run_ctx(ctx: string, code: string): string;

/**
 * Run Osmia code with a custom JSON context
 *
 * # Arguments
 * * `ctx` - The Osmia context as a string (JSON)
 * * `code` - The Osmia code to run as a string
 *
 * # Returns
 * * `Result<String, String>` - The result of the Osmia code, or an error message
 */
export function run_json_ctx(ctx: string, code: string): string;

/**
 * Run Osmia code with a custom YAML context
 *
 * # Arguments
 * * `ctx` - The Osmia context as a string (YAML)
 * * `code` - The Osmia code to run as a string
 *
 * # Returns
 * * `Result<String, String>` - The result of the Osmia code, or an error message
 */
export function run_yaml_ctx(ctx: string, code: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly run: (a: number, b: number) => [number, number, number, number];
    readonly run_ctx: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly run_json_ctx: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly run_yaml_ctx: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly ctx_json_dump: (a: number, b: number) => [number, number, number, number];
    readonly ctx_json_dump_variable: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
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
