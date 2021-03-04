use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn error(a: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(a: &str);
    #[wasm_bindgen(js_name = encodeURIComponent)]
    pub fn encode_uri(a: &str) -> JsValue;
}