use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
  pub fn alert(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn log(value: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => ($crate::utils::log(&format_args!($($t)*).to_string()))
}

pub(crate) use console_log;
