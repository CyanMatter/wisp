mod events;
mod get_element;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{
    Document,
    HtmlParagraphElement,
    HtmlTextAreaElement
};

#[wasm_bindgen]
pub fn boot() {
    utils::set_panic_hook();

    let doc = get_element::document!();
    
    add_event_listeners(&doc);
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
