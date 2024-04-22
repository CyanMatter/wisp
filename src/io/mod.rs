mod response;

use futures::stream::{AbortHandle, AbortRegistration, Abortable};
#[cfg(debug_assertions)]
use leptos::logging::log;
use leptos::{
    ev::{Event, InputEvent},
    *,
};
#[cfg(debug_assertions)]
use std::error::Error as _;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{wasm_bindgen::closure::Closure, AddEventListenerOptions};

// Function to reverse a string
fn reverse(input: String) -> String {
    input.chars().rev().collect()
}

/// Receptacle component for an input field.
#[component]
fn Receptacle<F>(
    /// Callback function that is to be called whenever an input event triggers on the receptacle.
    /// You'll likely want this callback function to assign the input value to some WriteSignal.
    on_input: F,
) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    // Render a textarea with autofocus and placeholder text
    view! {
        <textarea
            id="receptacle"
            autofocus=true
            placeholder="Type something!"
            on:input=on_input
        ></textarea>
    }
}

/// InputOutput component for main functionality.
#[component]
pub fn InputOutput() -> impl IntoView {
    // Create a signal to store the output string
    let (output, set_output) = create_signal(String::new());

    // Options for adding event listener (only listen once)
    let mut binding = AddEventListenerOptions::new();
    let once = binding.once(true).to_owned();

    // Callback function for input events
    let on_input = move |ev: Event| {
        // Convert event to InputEvent and check if it's a composing event (skip those)
        let iev = ev.unchecked_into::<InputEvent>();
        if iev.is_composing() {
            return;
        }

        // Create an abort handle and registration for aborting the expensive task
        let (handle, reg) = AbortHandle::new_pair();
        let abort = Closure::once_into_js(Box::new(move || {
            handle.abort();
        }) as Box<dyn FnOnce()>);

        // Add event listener to the receptacle with abort functionality
        iev.target()
            .unwrap()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "input",
                abort.as_ref().unchecked_ref(),
                once.as_ref(),
            )
            .expect("could not add an event listener to the receptacle");

        // Spawn a local task to run the expensive task with abort handling
        spawn_local(run_abortable(reg, iev, set_output));
    };

    // Render the Receptacle and Response components
    view! {
        <Receptacle on_input=on_input/>
        <response::ResponseList output=output/>
    }
}

// Simulate an expensive task that takes some time
async fn some_expensive_task(msg: String) -> Option<String> {
    const N: u32 = 3;
    #[cfg(debug_assertions)]
    log!("====== counting to {N} =======");
    for _i in 0..N {
        #[cfg(debug_assertions)]
        log!("before sleep i is {_i}");
        let delay = 1000;
        async_std::task::sleep(Duration::from_millis(delay as u64)).await;
    }
    #[cfg(debug_assertions)]
    log!("before sleep i is {N}\n========= all done! =========");

    // Return a formatted string with input type and reversed target value
    Some(msg)
}

// Simulate an easy task
async fn some_easy_task(msg: String) -> Option<String> {
    Some(msg)
}

// Run the expensive task with abort handling
async fn run_abortable(reg: AbortRegistration, iev: InputEvent, set_output: WriteSignal<String>) {
    // Wrap the expensive task "some_expensive_task" with abort functionality
    // using the provided registration handle "reg" to allow cancellation.
    let msg = format!(
        "{}\t{}",
        iev.input_type(),
        reverse(event_target_value(&iev))
    );
    let fut = Abortable::new(some_easy_task(msg), reg);
    // let fut = Abortable::new(some_expensive_task(msg), reg);

    // Await the completion of the wrapped task.
    // In case of an abortion (`Err(_aborted)`), handle it with a closure:
    let response = fut.await.unwrap_or_else(|_aborted| {
        // If there's a specific error source (`source`) causing the abort, log it (debug mode only).
        #[cfg(debug_assertions)]
        if let Some(source) = _aborted.source() {
            log!("{source}");
        } else {
            // Otherwise, log a generic message about the task being aborted.
            log!("task aborted")
        }

        // Since the task was aborted, return None to indicate no valid response.
        None
    });

    // Update the output signal with the response from the task (or an empty string if aborted).
    set_output.set(response.unwrap_or(String::new()));
}
