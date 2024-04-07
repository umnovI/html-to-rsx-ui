#![allow(non_snake_case)]

use crate::ui::Interface;
use dioxus::prelude::*;
use regex::Regex;

struct Replacer {
    regex: Regex,
    replace_with: String,
}

struct Re {
    content: Regex,
    word: Regex,
    element: Regex,
    class: Regex,
    closing_element: Regex,
    self_closing: Regex,
    r#type: Regex,
    placeholder: Regex,
    endline: Regex,
    spaces: Regex,
}

impl Re {
    fn new() -> Self {
        Self {
            content: Regex::new(r#"(<.*?>)(.*?)(<)"#).unwrap(),
            word: Regex::new(r#"(\s)(\w+\W+?)(\n)"#).unwrap(),
            element: Regex::new(r#"<(?P<tag_name>\w+)(?P<content>.*?)??/?>"#).unwrap(),
            class: Regex::new(r#"(class=(?P<classes>".*?"))"#).unwrap(),
            r#type: Regex::new(r#"(type=(?P<content>".*?"))"#).unwrap(),
            placeholder: Regex::new(r#"(placeholder=(?P<content>".*?"))"#).unwrap(),
            closing_element: Regex::new(r#"(</(?P<tag_name>\w+)>)"#).unwrap(),
            self_closing: Regex::new(r#"/>"#).unwrap(),
            endline: Regex::new(r#"(,)\s*(})"#).unwrap(),
            spaces: Regex::new(r#"\s{2,}"#).unwrap(),
        }
    }
}

#[allow(unused_mut)]
#[component]
pub fn App() -> Element {
    // So we don't compile regexes every time we call translate
    let re = Re::new();
    let mut data = use_signal(|| "".to_string());
    let mut translated_data = use_signal(|| "".to_string());
    log::info!("Data value: {}", data);
    if !data.read().is_empty() {
        translated_data.set(translate(data(), &re));
    }

    rsx! { Interface { data, translated_data } }
}

fn translate(data: String, re: &Re) -> String {
    let out = {
        let quote_words = re.content.replace_all(&data, "$1 \"$2\" $3");
        let l_word = re.word.replace_all(&quote_words, "$1 \"$2\" $3");
        let self_closing = re.self_closing.replace_all(&l_word, "> </replace_me>");
        let clean_tag = re
            .element
            .replace_all(&self_closing, "$tag_name { $content");
        let clean_class = re.class.replace_all(&clean_tag, "class: $classes, ");
        let clean_type = re.r#type.replace_all(&clean_class, "r#type: $content, ");
        let clean_placeholder = re
            .placeholder
            .replace_all(&clean_type, "placeholder: $content, ");
        let close_tag = re.closing_element.replace_all(&clean_placeholder, "}");
        close_tag.into_owned()
    };
    out
}
