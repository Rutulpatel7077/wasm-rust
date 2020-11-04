extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}


// hello world for fast_search
#[wasm_bindgen]
pub fn fast_search(array:Vec<i32>, name: i32) {
    let array_iter = array.into_iter();
    array_iter.filter(|&x| x == name);
}