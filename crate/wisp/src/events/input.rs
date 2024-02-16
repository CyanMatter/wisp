use crate::utils::log;
use gloo_events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{
  HtmlParagraphElement,
  HtmlTextAreaElement,
  InputEvent,
};

macro_rules! do_stuff {
  ($receptacle:expr, $target:expr) => {
    $target.set_text_content(Some(&format!("len: {len}\ncontent: {content}",
      len = $receptacle.text_length().to_string(),
      content = $receptacle.value())));
  };
}

#[wasm_bindgen]
pub fn on_input_show_content(receptacle: HtmlTextAreaElement, target: HtmlParagraphElement) {

  let on_input = EventListener::new(&receptacle.clone(), "input", move |event| {

    let input_event = event.clone()
      .dyn_into::<InputEvent>()
      .unwrap();

    // Idle if input is not yet completed
    if input_event.is_composing() {
      return;
    }

    // do_stuff!(receptacle, target);
    hard_task(&receptacle.clone(), &target.clone())
  });

  on_input.forget();
}

fn hard_task(receptacle: &HtmlTextAreaElement, target: &HtmlParagraphElement) {
  log(&String::from("Check"));

  let s1 = "Executing hard task...";
  target.set_text_content(Some(s1));

  let s2 = format!("Executed hard task\nContent: {}", receptacle.value());
  target.set_text_content(Some(&s2));
}
