use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDivElement, HtmlTextAreaElement};

const ID_RECEPTACLE: &str = "receptacle";
const ID_TARGET: &str = "target";
const SELECTOR_RESPONSES: &str = "div.response";

pub fn by_id(id: &str) -> Option<Element> {
    let selector = format!("#{}", id);

    leptos::document()
        .query_selector(&selector)
        .unwrap()
}

pub fn responses() -> Vec<HtmlDivElement> {
    let nodes = leptos::document()
        .query_selector_all(SELECTOR_RESPONSES)
        .unwrap();

    (0..nodes.length())
        .map(|i| {
            nodes
                .item(i)
                .unwrap()
                .unchecked_into::<HtmlDivElement>()
        })
        .collect()
}

pub fn target() -> Option<HtmlDivElement> {
    if let Some(target) = by_id(ID_TARGET) {
        return Some(target.unchecked_into::<HtmlDivElement>());
    }
    None
}

pub fn receptacle() -> Option<HtmlTextAreaElement> {
    if let Some(receptacle) = by_id(ID_RECEPTACLE) {
        return Some(receptacle.unchecked_into::<HtmlTextAreaElement>());
    }
    None
}
