use std::ffi::OsStr;

use ansi_to_html::convert;
use diatom::{Interpreter, IoWrite};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_version() -> String {
    diatom::VERSION.to_string()
}

struct JsOutputStream<'a> {
    callback: &'a js_sys::Function,
}

impl<'a> IoWrite for JsOutputStream<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let parameter = JsValue::from_str(
            std::str::from_utf8(buf).unwrap_or("Output is not valid utf8 string\n"),
        );
        let this = JsValue::null();
        let _ = self.callback.call1(&this, &parameter);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[wasm_bindgen]
pub fn exec(code: &str, f: &js_sys::Function) -> String {
    let stream = JsOutputStream { callback: f };
    let mut interpreter = Interpreter::with_color(stream);
    let result = match interpreter.exec(code, OsStr::new("<playground>")) {
        Ok(_) => return String::new(),
        Err(s) => s,
    };
    convert(&result, true, true)
        .unwrap_or_else(|_| "Internal error: Failed to convert ansi output".to_string())
}
