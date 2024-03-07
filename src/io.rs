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

fn reverse(input: String) -> String {
    input.chars().rev().collect()
}

#[component]
fn Receptacle<F>(on_input: F) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    view! {
        <textarea
            id="receptacle"
            autofocus=true
            placeholder="Type something!"
            on:input=on_input
        ></textarea>
    }
}

#[component]
fn Response(output: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="response">
            <span>{output}</span>
        </div>
    }
}

#[component]
pub fn InputOutput() -> impl IntoView {
    let (output, set_output) = create_signal(String::new());

    let mut binding = AddEventListenerOptions::new();
    let once = binding.once(true).to_owned();

    let on_input = move |ev: Event| {
        let iev = ev.unchecked_into::<InputEvent>();
        if iev.is_composing() {
            return;
        }
        let (handle, reg) = AbortHandle::new_pair();
        let abort = Closure::wrap(Box::new(move || {
            handle.abort();
        }) as Box<dyn FnMut()>);
        iev.target()
            .unwrap()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "input",
                abort.as_ref().unchecked_ref(),
                once.as_ref(),
            )
            .expect("could not add abortion event listener to receptacle");
        abort.forget();
        spawn_local(run_abortable(reg, iev, set_output));
    };

    view! {
        <Receptacle on_input=on_input/>
        <Response output=output/>
    }
}

async fn some_expensive_task(iev: InputEvent) -> Option<String> {
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
    Some(format!(
        "{}\t{}",
        iev.input_type(),
        reverse(event_target_value(&iev))
    ))
}

async fn run_abortable(reg: AbortRegistration, iev: InputEvent, set_output: WriteSignal<String>) {
    let fut = Abortable::new(some_expensive_task(iev), reg);

    let response = fut.await.unwrap_or_else(|_aborted| {
        #[cfg(debug_assertions)]
        if let Some(source) = _aborted.source() {
            log!("{source}");
        } else {
            log!("task aborted")
        }
        None
    });

    set_output.set(response.unwrap_or(String::new()));
}
