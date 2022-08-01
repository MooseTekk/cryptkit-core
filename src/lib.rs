use wasm_bindgen::prelude::*;
use std::str;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn ascii85_encode(input: &str) -> String {
    ascii85::encode(input.as_bytes())
}
#[wasm_bindgen]
pub fn ascii85_decode(input: &str) -> String {
    str::from_utf8(
        &ascii85::decode(input).unwrap()
    ).unwrap_or("Invalid Input").to_string()
}