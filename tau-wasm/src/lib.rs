use wasm_bindgen::prelude::*;
use tau_core::run_source;
use console_error_panic_hook;

#[wasm_bindgen(start)]
fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn execute(source: &str) -> String {
    match run_source(source) {
        Ok(_) => "Execution successful".into(),
        Err(e) => format!("Error: {}", e),
    }
}
