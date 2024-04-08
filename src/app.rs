#![allow(non_snake_case)]
use anyhow::{Context, Result};
use dioxus::prelude::*;
use html_parser::Dom;

use crate::ui::Interface;

#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    Err,
    Ok,
}

#[derive(PartialEq, Debug, Clone)]
pub struct App {
    pub html: Signal<String>,
    pub rsx: Signal<String>,
    pub status: Signal<Status>,
    pub error_data: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            html: use_signal(|| "".to_string()),
            rsx: use_signal(|| "".to_string()),
            status: use_signal(|| Status::Ok),
            error_data: None,
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut app = App::new();

    if !app.html.read().is_empty() {
        let res = translate((app.html)());
        match res {
            Ok(val) => app.rsx.set(val),
            Err(err) => {
                app.status.set(Status::Err);
                app.error_data = Some(err.to_string());
            }
        };
    }

    rsx! { Interface { app } }
}

fn translate(data: String) -> Result<String> {
    let dom = Dom::parse(data.trim()).with_context(|| "could not parse html")?;
    let body = rsx_rosetta::rsx_from_html(&dom);
    let out = dioxus_autofmt::write_block_out(body).with_context(|| "could not format output")?;
    Ok(out.trim().to_string())
}
