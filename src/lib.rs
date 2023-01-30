use ansi_to_html::convert;
use diatom::Interpreter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_version() -> String {
    diatom::VERSION.to_string()
}

#[wasm_bindgen]
pub fn compile(code: &str) -> String {
    let mut interpreter = Interpreter::new();
    let result = match interpreter.exec(code, true) {
        Ok(s) | Err(s) => s,
    };
    convert(&result, true, true)
        .unwrap_or_else(|_| "Internal error: Failed to convert ansi output".to_string())
}
