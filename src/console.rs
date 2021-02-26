use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn error(a: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(a: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::console::log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => (crate::console::error(&format_args!($($t)*).to_string()))
}
