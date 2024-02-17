use crate::utils::console_log;
use futures::stream::{AbortHandle, Abortable};
use gloo_events::EventListener;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlParagraphElement, HtmlTextAreaElement, InputEvent};

#[wasm_bindgen]
pub fn on_input_show_content(receptacle: HtmlTextAreaElement) {
  let listener = EventListener::new(&receptacle, "input", move |event| {
    let event = event.dyn_ref::<InputEvent>().unwrap_throw();

    // Idle if input is not yet completed
    if event.is_composing() {
      return;
    }

    let (abort_handle, abort_registration) = AbortHandle::new_pair();

    // Add a listener that aborts the task next time the content of the receptacle changes
    EventListener::once(&event.target().unwrap(), "input", move |_event| {
      abort_handle.abort();
    })
    .forget();

    // Create a future that performs a hard task and can be aborted
    spawn_local(async {
      let fut = Abortable::new(
        answer_to_life_the_universe_and_everything(),
        abort_registration,
      );
      let optional_result = fut.await.unwrap_or_else(|aborted| {
        console_log!("{aborted}");
        None
      });

      if let Some(result) = optional_result {
        console_log!("{result}");
      }
    });
  });
  listener.forget();
}

async fn answer_to_life_the_universe_and_everything() -> Option<i32> {
  console_log!("====== counting to 10 =======");
  for i in 0..10 {
    console_log!("before sleep i is {}", i);
    let delay = 1000;
    async_std::task::sleep(Duration::from_millis(delay as u64)).await;
  }
  console_log!("========= all done! =========");
  Some(42)
}
