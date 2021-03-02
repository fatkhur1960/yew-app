use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn error(a: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(a: &str);
    #[wasm_bindgen(js_namespace = toastr, js_name = "info")]
    pub fn notify(msg: &str);
    #[wasm_bindgen(js_namespace = toastr, js_name = "error")]
    pub fn notify_error(msg: &str);
    #[wasm_bindgen(js_namespace = toastr, js_name = "warning")]
    pub fn notify_warning(msg: &str);
    #[wasm_bindgen(js_namespace = toastr, js_name = "success")]
    pub fn notify_success(msg: &str);
    #[wasm_bindgen(js_namespace = toastr, js_name = "clear")]
    pub fn notify_clear(msg: &str);
}