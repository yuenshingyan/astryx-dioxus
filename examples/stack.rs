use astryx_dioxus::STYLES;
use dioxus::prelude::*;

#[path = "showcase/stack.rs"]
mod stack_examples;

use stack_examples::StackExamples;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Stack offline verification" }
        document::Style { "{STYLES}" }
        main { "data-astryx-theme": "astryx", StackExamples {} }
    }
}
