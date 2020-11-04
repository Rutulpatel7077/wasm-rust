extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}


// hello world for fast_search
#[wasm_bindgen]
pub fn fast_search(array, name: &str) {
    let array_iter = array.into_iter();
    array_iter.filter(|&&x| x == name);
}