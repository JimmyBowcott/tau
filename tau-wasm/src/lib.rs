use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use tau_core::{output::Output, run_source};
use console_error_panic_hook;

thread_local! {
    static BUFFER: RefCell<String> = RefCell::new(String::new());
}

struct WasmOutput;

impl Output for WasmOutput {
    fn write(& mut self, s: &str) {
        BUFFER.with(|b| b.borrow_mut().push_str(s));
    }
}

#[wasm_bindgen(start)]
fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn execute(source: &str) -> String {
    BUFFER.with(|b| b.borrow_mut().clear());
    let mut out = WasmOutput;

    match run_source(source, &mut out) {
        Ok(_) => BUFFER.with(|b| b.borrow().clone()),
        Err(e) => e.to_string(),
    }
}
