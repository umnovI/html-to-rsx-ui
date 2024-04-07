#![allow(non_snake_case)]

use std::process::Command;

use crate::ui::Interface;
use dioxus::prelude::*;

#[allow(unused_mut)]
#[component]
pub fn App() -> Element {
    let mut data = use_signal(|| "".to_string());
    let mut translated_data = use_signal(|| "".to_string());
    log::info!("Data value: {}", data);
    if !data.read().is_empty() {
        translated_data.set(translate(data()));
    }

    rsx! { Interface { data, translated_data } }
}

fn translate(data: String) -> String {
    let output = {
        Command::new("nu")
            .args(["--commands", &format!("dx translate -r '{}'", data)])
            .output()
            .unwrap()
    };
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
