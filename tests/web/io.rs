use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    Event, HtmlDivElement, HtmlSpanElement, HtmlTextAreaElement,
};

use super::*;
use crate::page;

struct Response {
    span: HtmlSpanElement,
    content: String,
}
impl Response {
    fn from_html(response: HtmlDivElement) -> Response {
        let span = response
            .query_selector("span")
            .expect("no span found in response component")
            .unwrap()
            .unchecked_into::<HtmlSpanElement>();

        let content = span
            .text_content()
            .expect("no text content found in response component");

        Response { span, content }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn receptacle_is_only_textarea() {
    page::setup_io();

    let textareas = leptos::document().query_selector_all("textarea").unwrap();

    assert_eq!(textareas.length(), 1);

    let textarea = textareas
        .get(0)
        .unwrap()
        .unchecked_into::<HtmlTextAreaElement>();
    let receptacle = get::receptacle().unwrap();

    assert_eq!(textarea, receptacle);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn receptacle_has_autofocus() {
    page::setup_io();

    let textarea = get::receptacle().unwrap();
    assert!(textarea.autofocus());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn receptacle_has_placeholder() {
    page::setup_io();

    let textarea = get::receptacle().unwrap();
    assert_eq!(textarea.placeholder(), "Type something!");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn target_initializes_singularly() {
    page::setup_io();

    let responses = get::responses();
    assert_eq!(responses.len(), 1);

    let target = get::target();
    assert_eq!(*responses.first().unwrap(), target);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn target_initializes_empty() {
    page::setup_io();

    let target = Response::from_html(get::target());
    assert_eq!(target.content, "");
}
