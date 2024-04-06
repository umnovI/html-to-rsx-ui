#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="/tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "container mx-auto mt-4",
            div { class: "grid grid-cols-2 gap-4",
                textarea {
                    id: "text-to-parse",
                    placeholder: "Type here",
                    class: "input input-bordered min-h-96 w-full text-start resize-none pt-2"
                }
                textarea {
                    id: "parsed-text",
                    placeholder: "Type here",
                    class: "input input-bordered min-h-96 w-full resize-none pt-2"
                }
            }
            div { class: "flex items-center justify-center mt-4",
                button { class: "btn", "Translate to rsx" }
            }
        }
    }
}
