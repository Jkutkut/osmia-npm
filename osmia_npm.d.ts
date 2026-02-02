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
