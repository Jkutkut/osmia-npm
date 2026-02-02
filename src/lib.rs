use wasm_bindgen::prelude::*;
use osmia::Osmia;

#[cfg(test)]
mod test;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn add_npm_ctx(osmia: &mut Osmia) {
	osmia.run_code(&format!("{{{{ _OSMIA_NPM_VERSION = \"{}\" }}}}", VERSION)).unwrap();
}

fn run_osmia(mut osmia: Osmia, code: &str) -> Result<String, String> {
	add_npm_ctx(&mut osmia);
	osmia.run_code(code)
}

/// Run Osmia code with the default context
///
/// # Arguments
/// * `code` - The Osmia code to run as a string
///
/// # Returns
/// * `Result<String, String>` - The result of the Osmia code, or an error message
#[wasm_bindgen]
pub fn run(code: &str) -> Result<String, String> {
	run_osmia(Osmia::default(), &code)
}

/// Run Osmia code with a custom context
///
/// # Arguments
/// * `ctx` - The Osmia context as a string (JSON)
/// * `code` - The Osmia code to run as a string
///
/// # Returns
/// * `Result<String, String>` - The result of the Osmia code, or an error message
#[wasm_bindgen]
pub fn run_ctx(ctx: &str, code: &str) -> Result<String, String> {
	run_json_ctx(ctx, code)
}

/// Run Osmia code with a custom JSON context
///
/// # Arguments
/// * `ctx` - The Osmia context as a string (JSON)
/// * `code` - The Osmia code to run as a string
///
/// # Returns
/// * `Result<String, String>` - The result of the Osmia code, or an error message
#[wasm_bindgen]
pub fn run_json_ctx(ctx: &str, code: &str) -> Result<String, String> {
	run_osmia(Osmia::try_from_json(ctx)?, &code)
}

/// Run Osmia code with a custom YAML context
///
/// # Arguments
/// * `ctx` - The Osmia context as a string (YAML)
/// * `code` - The Osmia code to run as a string
///
/// # Returns
/// * `Result<String, String>` - The result of the Osmia code, or an error message
#[wasm_bindgen]
pub fn run_yaml_ctx(ctx: &str, code: &str) -> Result<String, String> {
	run_osmia(Osmia::try_from_yaml(ctx)?, &code)
}

enum DumpCtxRequest {
	JSON(String),
	YAML(String),
}

impl DumpCtxRequest {
	fn as_json(ctx: Option<String>) -> Option<Self> {
		ctx.map(
			|ctx| Self::JSON(ctx)
		)
	}

	fn as_yaml(ctx: Option<String>) -> Option<Self> {
		ctx.map(
			|ctx| Self::YAML(ctx)
		)
	}
}

fn ctx2osmia(ctx: Option<DumpCtxRequest>) -> Result<Osmia, String> {
	let mut osmia = match ctx {
		None => Osmia::default(),
		Some(ctx) => match ctx {
			DumpCtxRequest::JSON(ctx) => Osmia::try_from_json(&ctx)?,
			DumpCtxRequest::YAML(ctx) => Osmia::try_from_yaml(&ctx)?,
		}
	};
	add_npm_ctx(&mut osmia);
	Ok(osmia)
}

/// Get the Osmia context, based on a given initial context.
///
/// # Arguments
/// * `ctx` - The initial context as a string with JSON format
///
/// # Returns
/// * `Result<String, String>` - The Osmia context as a string in JSON format, or an error message
#[wasm_bindgen]
pub fn ctx_json_dump(ctx: Option<String>) -> Result<String, String> {
	Ok(ctx2osmia(DumpCtxRequest::as_json(ctx))?.ctx_json_dump())
}

/// Get the Osmia context, based on a given initial context.
///
/// # Arguments
/// * `ctx` - The initial context as a string with YAML format
///
/// # Returns
/// * `Result<String, String>` - The Osmia context as a string in JSON format, or an error message
#[wasm_bindgen]
pub fn ctx_yaml_dump(ctx: Option<String>) -> Result<String, String> {
	Ok(ctx2osmia(DumpCtxRequest::as_yaml(ctx))?.ctx_json_dump())
}

/// Get a variable from the Osmia context, based on a given initial context.
///
/// # Arguments
/// * `var` - The name of the variable to get
/// * `ctx` - The initial context as a string with JSON format
///
/// # Returns
/// * `Result<String, String>` - The value of the variable in JSON format, or an error message
#[wasm_bindgen]
pub fn ctx_json_dump_variable(var: &str, ctx: Option<String>) -> Result<String, String> {
	ctx2osmia(DumpCtxRequest::as_json(ctx))?.ctx_json_dump_variable(var)
}

/// Get a variable from the Osmia context, based on a given initial context.
///
/// # Arguments
/// * `var` - The name of the variable to get
/// * `ctx` - The initial context as a string with YAML format
///
/// # Returns
/// * `Result<String, String>` - The value of the variable in JSON format, or an error message
#[wasm_bindgen]
pub fn ctx_yaml_dump_variable(var: &str, ctx: Option<String>) -> Result<String, String> {
	ctx2osmia(DumpCtxRequest::as_yaml(ctx))?.ctx_json_dump_variable(var)
}
