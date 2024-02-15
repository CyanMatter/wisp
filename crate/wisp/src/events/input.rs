use gloo_events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{
  HtmlParagraphElement,
  HtmlTextAreaElement,
  InputEvent,
};

macro_rules! do_stuff {
  ($text_area:expr, $message:expr) => {
    $message.set_text_content(Some(&format!("len: {len}\ncontent: {content}",
      len = $text_area.text_length().to_string(),
      content = $text_area.value())));
  };
}

#[wasm_bindgen]
pub fn on_input_show_content(text_area: HtmlTextAreaElement, message: HtmlParagraphElement) {

  let on_input = EventListener::new(&text_area.clone(), "input", move |event| {

    let input_event = event.clone()
      .dyn_into::<InputEvent>()
      .unwrap();

    // Idle if input is not yet completed
    if input_event.is_composing() {
      return;
    }

    do_stuff!(text_area, message);
  });

  on_input.forget();
}
