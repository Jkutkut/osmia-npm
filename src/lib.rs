use wasm_bindgen::prelude::*;
use osmia::Osmia;

#[wasm_bindgen]
pub fn run(code: String) -> Result<String, String> {
	Osmia::default().run_code(&code)
}
