#![allow(non_snake_case)]

use crate::ui::Interface;
use dioxus::prelude::*;
use regex::Regex;

struct Replace {
    regex: Regex,
    replace_with: String,
}

struct Re {
    content: Replace,
    word: Replace,
    element: Replace,
    class: Replace,
    closing_element: Replace,
    self_closing: Replace,
    r#type: Replace,
    placeholder: Replace,
    endline: Replace,
    spaces: Replace,
}

impl Re {
    fn new() -> Self {
        Self {
            content: Replace {
                regex: Regex::new(r#"(<.*?>)(.*?)(<)"#).unwrap(),
                replace_with: "$1 \"$2\" $3".to_string(),
            },
            word: Replace {
                regex: Regex::new(r#"(\s)(\w+\W+?)(\n)"#).unwrap(),
                replace_with: "$1 \"$2\" $3".to_string(),
            },
            element: Replace {
                regex: Regex::new(r#"<(?P<tag_name>\w+)(?P<content>.*?)??/?>"#).unwrap(),
                replace_with: "$tag_name { $content".to_string(),
            },
            class: Replace {
                regex: Regex::new(r#"(class=(?P<classes>".*?"))"#).unwrap(),
                replace_with: "class: $classes, ".to_string(),
            },
            r#type: Replace {
                regex: Regex::new(r#"(type=(?P<content>".*?"))"#).unwrap(),
                replace_with: "r#type: $content, ".to_string(),
            },
            placeholder: Replace {
                regex: Regex::new(r#"(placeholder=(?P<content>".*?"))"#).unwrap(),
                replace_with: "placeholder: $content, ".to_string(),
            },
            closing_element: Replace {
                regex: Regex::new(r#"(</(?P<tag_name>\w+)>)"#).unwrap(),
                replace_with: "}".to_string(),
            },
            self_closing: Replace {
                regex: Regex::new(r#"/>"#).unwrap(),
                replace_with: "> </replace_me>".to_string(),
            },
            endline: Replace {
                regex: Regex::new(r#"(,)\s*(})"#).unwrap(),
                replace_with: "$1 $2".to_string(),
            },
            spaces: Replace {
                regex: Regex::new(r#"\s{2,}"#).unwrap(),
                replace_with: " ".to_string(),
            },
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
        let quote_words = re
            .content
            .regex
            .replace_all(&data, &re.content.replace_with);
        let l_word = re
            .word
            .regex
            .replace_all(&quote_words, &re.word.replace_with);
        let self_closing = re
            .self_closing
            .regex
            .replace_all(&l_word, &re.self_closing.replace_with);
        let clean_tag = re
            .element
            .regex
            .replace_all(&self_closing, &re.element.replace_with);
        let clean_class = re
            .class
            .regex
            .replace_all(&clean_tag, &re.class.replace_with);
        let clean_type = re
            .r#type
            .regex
            .replace_all(&clean_class, &re.r#type.replace_with);
        let clean_placeholder = re
            .placeholder
            .regex
            .replace_all(&clean_type, &re.placeholder.replace_with);
        let close_tag = re
            .closing_element
            .regex
            .replace_all(&clean_placeholder, &re.closing_element.replace_with);
        close_tag.into_owned()
    };
    out
}
