use astryx_dioxus::STYLES;
use dioxus::prelude::*;

#[path = "showcase/stack.rs"]
mod stack_examples;

use stack_examples::StackExamples;

const FIGTREE: Asset = asset!("/assets/fonts/Figtree-latin.woff2");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Stack offline verification" }
        document::Style { "{STYLES}" }
        document::Style { "@font-face {{font-family:Figtree;src:url('{FIGTREE}') format('woff2');font-style:normal;font-weight:300 900;font-display:swap;}}" }
        main { "data-astryx-theme": "astryx", StackExamples {} }
    }
}
