#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

mod app;
mod ui;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    #[cfg(debug_assertions)]
    let cfg = dioxus::desktop::Config::new().with_custom_head(
        r#"
        <link rel="stylesheet" href="/tailwind.css">
        "#
        .to_string(),
    );

    #[cfg(not(debug_assertions))]
    let cfg = dioxus::desktop::Config::new().with_custom_head(
        r#"
        <link rel="stylesheet" href="/tailwind.min.css">
        "#
        .to_string(),
    );

    LaunchBuilder::desktop().with_cfg(cfg).launch(app::App);
}
