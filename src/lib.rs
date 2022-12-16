use std::ffi::OsStr;

use wasm_bindgen::prelude::*;
use ansi_to_html::convert;
use diatom::Parser;

#[wasm_bindgen]
pub fn get_version() -> String{
    diatom::VERSION.to_string()
}

#[wasm_bindgen]
pub fn compile(code: &str) -> String {
    let mut parser = Parser::new();
    let ast = parser.parse_str(OsStr::new("playground"), code);
    let mut result = ast.diagnoser.render(true);
    if ast.diagnoser.error_count() == 0 {
        result += &format!("{:?}", ast.statements);
    }
    convert(&result, true, true).unwrap_or_else(|_|"Internal error: Failed to convert ansi output".to_string())
}
