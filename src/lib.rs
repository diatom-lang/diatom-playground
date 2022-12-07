use std::ffi::OsStr;

use wasm_bindgen::prelude::*;
use ansi_to_html::convert;
use diatom::Parser;

#[wasm_bindgen]
pub fn get_version() -> String{
    todo!()
}

#[wasm_bindgen]
pub fn compile(code: &str) -> String {
    let mut parser = Parser::new();
    parser.parse_str(OsStr::new("playground"), code);
    let result = if parser.diagnostic_count() > 0 {
        parser.render_diagnoses(true)
    } else {
        let ast = parser.get_incremental();
        format!("{:?}", ast.collect::<Vec<_>>())
    };
    convert(&result, true, true).unwrap_or_else(|_|"Internal error: Failed to convert ansi output".to_string())
}
