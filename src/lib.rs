pub mod io;

use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! { <io::InputOutput/> }
}