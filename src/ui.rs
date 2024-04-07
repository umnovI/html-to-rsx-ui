#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Interface(data: Signal<String>, translated_data: Signal<String>) -> Element {
    let mut draft = use_signal(|| "".to_string());
    let onclick = move |_| {
        log::info!("click");
        if !draft.read().is_empty() {
            log::info!("i have value");
            data.set(draft());
        }
    };
    rsx! {
        div { class: "container mx-auto mt-4",
            div { class: "grid grid-cols-2 gap-4",
                textarea {
                    id: "text-to-parse",
                    placeholder: "Your input",
                    class: "input input-bordered min-h-96 w-full text-start resize-none pt-2",
                    oninput: move |evt: Event<FormData>| {
                        draft.set(evt.value());
                        log::info!("{:?}", evt.value());
                    }
                }
                textarea {
                    id: "parsed-text",
                    placeholder: "Formatted output",
                    class: "input input-bordered min-h-96 w-full resize-none pt-2",
                    value: "{translated_data}"
                }
            }
            div { class: "flex items-center justify-center mt-4",
                button {
                    class: "btn",
                    prevent_default: "onclick",
                    onclick: onclick,
                    "Translate to rsx"
                }
            }
        }
    }
}
