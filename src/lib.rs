mod utils;

use sha1::{Digest, Sha1};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sha1(content: &str) -> String {
    let mut hasher = Sha1::new();

    hasher.update(content);

    let result = hasher.finalize();

    base16ct::lower::encode_string(&result)
}

#[wasm_bindgen]
pub fn base64_encode(content: &str) -> String {
    base64::encode(content)
}

#[wasm_bindgen]
pub fn base_sign(content: &str) -> String {
    sha1(base64_encode(content).as_str())
}
