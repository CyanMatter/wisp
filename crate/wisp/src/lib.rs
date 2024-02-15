mod events;
mod get_element;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{
    Document,
    HtmlParagraphElement,
    HtmlTextAreaElement
};

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);

//     #[wasm_bindgen(js_namespace = console)]
//     fn log(value: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     log(&format!("Hello, {}!", name));
// }

#[wasm_bindgen]
pub fn boot() {
    utils::set_panic_hook();

    let doc = get_element::document!();
    
    add_event_listeners(&doc);

    // let optional_text_area = get_element::by_id::<HtmlTextAreaElement>(&doc, "text-area");
    // let optional_message = get_element::by_id::<HtmlParagraphElement>(&doc, "msg-content");
    // if let (Some(text_area), Some(message)) = (optional_text_area, optional_message) {
    //     events::on_input_show_content(text_area, message);
    // }
}

fn add_event_listeners(doc: &Document) {
    add_on_input_show_content(doc);
}

fn add_on_input_show_content(doc: &Document) {
    let optional_receptacle = get_element::by_id::<HtmlTextAreaElement>(doc, "receptacle");
    let optional_target = get_element::by_id::<HtmlParagraphElement>(doc, "target");
    if let (Some(receptacle), Some(target)) = (optional_receptacle, optional_target) {
        events::on_input_show_content(receptacle, target);
    }
}