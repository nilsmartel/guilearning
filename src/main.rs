#![allow(non_snake_case)]

use dioxus::{html::style, prelude::*};

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let audience = "Nils";
    cx.render(rsx! (
        div {
            display: "flex",
            align_items: "end",
            justify_items: "flex-end",
            div {
                "Hello {audience}"
                div {
                    for i in 0..10 {
                        Button {
                            color: if i & 1 == 0 { "orange" } else { "lightblue"}
                            , value: i.to_string()
                        }
                    },

                    Button {
                        color: "red",
                        value: format!("hi")
                    }
                }

            }
        }
    ))
}

#[inline_props]
fn Button(cx: Scope, value: String, color: Option<&'static str>) -> Element {
    render! {
        div {
            padding: "2px",
            margin: "2px",
            border_radius: "4px",
            color: "white",
            background_color: color.unwrap_or_default(),
            font_family: "monospace",
            "{value}"
        }
    }
}
