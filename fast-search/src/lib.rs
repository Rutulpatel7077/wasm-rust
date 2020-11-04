extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}


#[wasm_bindgen]
pub fn fast_search(name: &str) {
    log(&format!("Here is the log {}", name))
}