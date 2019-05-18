mod encoders;
mod utils;

#[cfg(test)]
mod tests;

use wasm_bindgen::prelude::*;
use encoders::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn encode_html(input: &str) -> String {
    HtmlEntityEncoder::encode(input)
}

#[wasm_bindgen]
pub fn encode_attribute(input: &str) -> String {
    HtmlAttributeEncoder::encode(input)
}

#[wasm_bindgen]
pub fn encode_css(input: &str) -> String {
    CssEncoder::encode(input)
}

#[wasm_bindgen]
pub fn encode_http_query(input: &str) -> String {
    HttpQueryEncoder::encode(input)
}

#[wasm_bindgen]
pub fn encode_javascript_variable(input: &str) -> String {
    JavascriptVariableEncoder::encode(input)
}
