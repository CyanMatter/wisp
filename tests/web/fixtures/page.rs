use leptos::*;
use leptos_tutorial::io::InputOutput;

use crate::remove;

pub fn setup_io() {
    remove::receptacle();
    remove::responses();

    mount_to_body(|| view! { <InputOutput/> });
}
