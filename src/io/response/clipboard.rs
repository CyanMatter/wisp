use leptos::{
    component,
    logging::warn,
    IntoView,
    spawn_local,
    view,
    window
};

#[component]
pub fn CopyButton(text: String) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            copy_to_clipboard(text.clone());
        }>
            "Copy"
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