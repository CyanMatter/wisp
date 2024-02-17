use crate::utils::console_log;
use futures::stream::{AbortHandle, AbortRegistration, Abortable};
use gloo_events::EventListener;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlParagraphElement, HtmlTextAreaElement, InputEvent};

#[cfg(debug_assertions)]
use std::error::Error;

#[wasm_bindgen]
pub fn on_input_show_content(receptacle: HtmlTextAreaElement, target: HtmlParagraphElement) {
  // Create an event listener to capture input events
  let listener = EventListener::new(&receptacle.clone(), "input", move |event| {
    // Get the new contents of the receptacle
    let message = receptacle.value();

    // Convert the event to an InputEvent
    let event = event.dyn_ref::<InputEvent>().unwrap_throw();

    // Ignore events if input is still being composed
    if event.is_composing() {
      return;
    }

    // Create a mechanism for aborting the task if needed
    let (handle, reg) = AbortHandle::new_pair();

    // Set up a listener to trigger task abortion on subsequent input
    EventListener::once(&event.target().unwrap(), "input", move |_event| {
      handle.abort();
    })
    .forget();

    // Spawn a new asynchronous task that can be aborted
    spawn_local(run_abortable(reg, message, target.clone()));
  });

  // Keep the listener alive indefinitely
  listener.forget();
}

async fn run_abortable(reg: AbortRegistration, message: String, target: HtmlParagraphElement) {
  // Wrap the long-running task in an Abortable future
  let fut = Abortable::new(reverse_that_takes_a_long_time(&message), reg);

  // Wait for the task to complete or be aborted
  let response = fut.await.unwrap_or_else(|_aborted| {
    // Log a message if the task was aborted
    #[cfg(debug_assertions)]
    if let Some(source) = _aborted.source() {
      console_log!("{source}");
    } else {
      console_log!("task aborted");
    }

    None
  });

  // If the task completed successfully, show the result
  target.set_text_content(response.as_deref());
}

async fn reverse_that_takes_a_long_time(message: &str) -> Option<String> {
  const N: u32 = 3;
  console_log!("====== counting to {N} =======");
  for i in 0..N {
    console_log!("before sleep i is {i}");
    let delay = 1000;
    async_std::task::sleep(Duration::from_millis(delay as u64)).await;
  }
  console_log!("before sleep i is {N}");
  console_log!("========= all done! =========");
  Some(message.chars().rev().collect())
}
