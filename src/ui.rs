#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::app::{App, Status};

#[component]
pub fn Interface(app: App) -> Element {
    let mut draft = use_signal(|| "".to_string());
    let err_status = app.status;
    let err_msg = app
        .error_data
        .unwrap_or_else(|| "Test message.".to_string());

    let onclick = move |_| {
        if !draft.read().is_empty() {
            app.html.set(draft());
        }
    };

    rsx! {
        div { class: "container mx-auto mt-4",
            if matches!((app.status)(), Status::Err) {
                ErrorAlert { err_status, err_msg }
            }
            div { class: "grid grid-cols-2 gap-4",
                textarea {
                    id: "text-to-parse",
                    placeholder: "Your input",
                    class: "input input-bordered min-h-96 w-full text-start resize-none pt-2",
                    oninput: move |evt: Event<FormData>| {
                        draft.set(evt.value());
                    }
                }
                textarea {
                    id: "parsed-text",
                    placeholder: "Formatted output",
                    class: "input input-bordered min-h-96 w-full resize-none pt-2",
                    value: "{app.rsx}"
                }
            }
            div { class: "flex items-center justify-center mt-4",
                button {
                    class: "btn",
                    prevent_default: "onclick",
                    onclick: onclick,
                    "Translate to rsx"
                }
                button {
                    class: "btn",
                    prevent_default: "onclick",
                    onclick: move |_| { app.status.set(Status::Err) },
                    "Make an error"
                }
            }
        }
    }
}

#[component]
fn ErrorAlert(err_status: Signal<Status>, err_msg: String) -> Element {
    rsx! {
        div { class: "fixed right-0", id: "alert-popup",
            div {
                role: "alert",
                class: "alert alert-error max-w-96 justify-end",
                span { "{err_msg}" }
                button {
                    r#type: "button",
                    class: "error-close",
                    "data-tip": "Close",
                    onclick: move |_| { err_status.set(Status::Ok) },
                    svg {
                        "fill": "none",
                        "xmlns": "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 14 14",
                        class: "w-3 h-3",
                        path {
                            "stroke-width": "2",
                            "stroke": "currentColor",
                            "stroke-linejoin": "round",
                            "d": "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6",
                            "stroke-linecap": "round"
                        }
                    }
                }
            }
        }
    }
}
