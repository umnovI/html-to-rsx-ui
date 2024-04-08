#![allow(non_snake_case)]

use copypasta::{ClipboardContext, ClipboardProvider};
use dioxus::prelude::*;
use std::{thread, time::Duration};

use crate::app::{App, Status};

#[component]
pub fn Interface(app: App) -> Element {
    let err_status = app.status;
    let err_msg = app.error_data;
    let rsx = app.rsx;

    let mut draft = use_signal(|| "".to_string());

    let onclick = move |_| {
        if !draft.read().is_empty() {
            app.html.set(draft());
        }
    };

    rsx! {
        div { class: "container mx-auto mt-4",
            if matches!(err_status(), Status::Err) {
                ErrorAlert { err_status, err_msg }
            }
            div { class: "grid grid-cols-2 gap-4",
                textarea {
                    id: "text-to-translate",
                    placeholder: "Your input",
                    class: "input input-bordered min-h-96 w-full text-start resize-none pt-2",
                    oninput: move |evt: Event<FormData>| {
                        draft.set(evt.value());
                    }
                }
                div {
                    ErrorBoundary {
                        handle_error: |error| {
                            rsx! {
                                "Could not initialize CopyBtn component: {error}"
                            }
                        },
                        div { class: "absolute top-auto right-8 p-2",
                            CopyBtn { rsx, err_status, err_msg }
                        }
                    }
                    textarea {
                        id: "translated-text",
                        placeholder: "Formatted output",
                        class: "input input-bordered min-h-96 w-full resize-none pt-2",
                        value: "{rsx}"
                    }
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

#[component]
fn ErrorAlert(err_status: Signal<Status>, err_msg: Signal<Option<String>>) -> Element {
    let msg = err_msg()?;

    rsx! {
        div { class: "fixed right-0", id: "alert-popup",
            div { class: "pr-4",
                div {
                    role: "alert",
                    class: "alert alert-error max-w-96 justify-end",
                    span { "{msg}" }
                    button {
                        r#type: "button",
                        class: "error-close",
                        "data-tip": "Close",
                        onclick: move |_| {
                            err_status.set(Status::Ok);
                        },
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
}

struct CopyBtn {
    copied: Signal<bool>,
    tip: Signal<String>,
}

impl CopyBtn {
    fn new() -> Self {
        Self {
            copied: use_signal(|| false),
            tip: use_signal(|| "copy".to_string()),
        }
    }
}

#[component]
fn CopyBtn(
    rsx: Signal<String>,
    err_status: Signal<Status>,
    err_msg: Signal<Option<String>>,
) -> Element {
    let mut ctx = ClipboardContext::new().throw()?;
    let mut copy = CopyBtn::new();

    rsx! {
        div {
            "data-tip": "{copy.tip}",
            class: "tooltip tooltip-left",
            onmouseleave: move |_| {
                thread::sleep(Duration::from_millis(200));
                copy.tip.set("copy".to_string());
                copy.copied.set(false);
            },
            button {
                class: "btn btn-square btn-sm",
                onclick: move |_| {
                    if !rsx().is_empty() {
                        ctx.set_contents(rsx().to_owned()).unwrap();
                        copy.copied.set(true);
                        copy.tip.set("copied".to_string());
                    } else {
                        copy.tip.set("nothing to copy".to_string());
                    }
                },
                if (copy.copied)() {
                    CopiedIcon {}
                } else {
                    CopyIcon {}
                }
            }
        }
    }
}

#[component]
fn CopyIcon() -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 32 32",
            "xmlns": "http://www.w3.org/2000/svg",
            class: "h-5 w-5 fill-current",
            path { "d": "M 16 3 C 14.742188 3 13.847656 3.890625 13.40625 5 L 6 5 L 6 28 L 26 28 L 26 5 L 18.59375 5 C 18.152344 3.890625 17.257813 3 16 3 Z M 16 5 C 16.554688 5 17 5.445313 17 6 L 17 7 L 20 7 L 20 9 L 12 9 L 12 7 L 15 7 L 15 6 C 15 5.445313 15.445313 5 16 5 Z M 8 7 L 10 7 L 10 11 L 22 11 L 22 7 L 24 7 L 24 26 L 8 26 Z" }
        }
    }
}

#[component]
fn CopiedIcon() -> Element {
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 0 32 32",
            class: "h-5 w-5 fill-current",
            path { "d": "M 16 2 C 14.742188 2 13.847656 2.890625 13.40625 4 L 5 4 L 5 29 L 27 29 L 27 4 L 18.59375 4 C 18.152344 2.890625 17.257813 2 16 2 Z M 16 4 C 16.554688 4 17 4.445313 17 5 L 17 6 L 20 6 L 20 8 L 12 8 L 12 6 L 15 6 L 15 5 C 15 4.445313 15.445313 4 16 4 Z M 7 6 L 10 6 L 10 10 L 22 10 L 22 6 L 25 6 L 25 27 L 7 27 Z M 21.28125 13.28125 L 15 19.5625 L 11.71875 16.28125 L 10.28125 17.71875 L 14.28125 21.71875 L 15 22.40625 L 15.71875 21.71875 L 22.71875 14.71875 Z" }
        }
    }
}
