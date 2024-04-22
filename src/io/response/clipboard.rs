use leptos::{
    component, logging::warn, spawn_local, view, window, IntoView, ReadSignal, SignalGetUntracked
};

#[component]
pub fn CopyResponseFromSignal(text: ReadSignal<String>) -> impl IntoView {
    view! {
        <button class="button-copy-response" on:click=move |_| {
            copy_to_clipboard(text.get_untracked());
        }>
            "Copy"
        </button>
    }
}

#[component]
pub fn CopyResponse(text: String) -> impl IntoView {
    view! {
        <button class="button-copy-response" on:click=move |_| {
            copy_to_clipboard(text.clone());
        }>
            "Copy"
        </button>
    }
}

#[component]
pub fn CopyShareableURLFromSignal(text: ReadSignal<String>) -> impl IntoView {
    view! {
        <button class="button-copy-shareable-url" on:click=move |_| {
            copy_to_clipboard(format!("TODO: Make shareable url. Response string: {}", text.get_untracked()));
        }>
            "Share"
        </button>
    }
}

#[component]
pub fn CopyShareableURL(text: String) -> impl IntoView {
    view! {
        <button class="button-copy-shareable-url" on:click=move |_| {
            copy_to_clipboard(format!("TODO: Make shareable url. Response string: {text}"));
        }>
            "Share"
        </button>
    }
}

fn copy_to_clipboard(text: String) {
    spawn_local(async move {
        let cb_option = window().navigator().clipboard();
        match cb_option {
            Some(cb) => {
                let _ = cb.write_text(&text);
            }
            None => {
                warn!("Failed to access clipboard");
            }
        };
    });
}