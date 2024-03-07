use wasm_bindgen_test::*;

pub mod io;

pub mod fixtures;
pub use fixtures::*;

wasm_bindgen_test_configure!(run_in_browser);