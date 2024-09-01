use wasm_bindgen::prelude::wasm_bindgen;
use crate::js_utils::set_panic_hook;


/// Initialize module
///
/// Should be called once after module is imported
#[wasm_bindgen]
pub fn init() {
    set_panic_hook()
}

/// My sample function
///
/// # Arguments
///
/// * `first`: some args
/// * `second`: another arg
///
/// returns: (just call alert)
///
/// # Examples
///
/// ```
///
/// ```
#[wasm_bindgen]
pub fn sample_shit(first: usize, second: usize) {
    crate::alert(&(crate::core::add(first, second)));
}