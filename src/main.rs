use leptos::mount_to_body;
use leptos_tutorial::App;

fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    mount_to_body(App);
}
