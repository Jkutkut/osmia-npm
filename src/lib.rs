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

#[wasm_bindgen]
pub fn run(code: &str) -> Result<String, String> {
	run_osmia(Osmia::default(), &code)
}

#[wasm_bindgen]
pub fn run_ctx(ctx: &str, code: &str) -> Result<String, String> {
	run_osmia(Osmia::try_from(ctx)?, &code)
}

fn ctx2osmia(ctx: Option<String>) -> Result<Osmia, String> {
	let mut osmia = match ctx {
		Some(ctx) => Osmia::try_from(ctx.as_str())?,
		None => Osmia::default(),
	};
	add_npm_ctx(&mut osmia);
	Ok(osmia)
}

#[wasm_bindgen]
pub fn ctx_json_dump(ctx: Option<String>) -> Result<String, String> {
	Ok(ctx2osmia(ctx)?.ctx_json_dump())
}

#[wasm_bindgen]
pub fn ctx_json_dump_variable(var: &str, ctx: Option<String>) -> Result<String, String> {
	ctx2osmia(ctx)?.ctx_json_dump_variable(var)
}
