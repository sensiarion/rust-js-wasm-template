#[cfg(feature = "js")]
use wasm_bindgen::prelude::wasm_bindgen;

mod core;

#[cfg(feature = "js")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


#[cfg(feature = "js")]
mod js_utils;

#[cfg(feature = "js")]
pub mod js_module;


#[cfg(feature = "python")]
pub mod python_module;
