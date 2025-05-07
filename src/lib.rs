use wasm_bindgen::prelude::*;
use osmia::Osmia;

#[cfg(test)]
mod test;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn run_osmia(mut osmia: Osmia, code: &str) -> Result<String, String> {
	osmia.run_code(&format!("{{{{ _OSMIA_NPM_VERSION = \"{}\" }}}}", VERSION)).unwrap();
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
