use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn md5(value: &str) -> String {
    let digest = md5::compute(value);

    format!("{:x}", digest)
}
