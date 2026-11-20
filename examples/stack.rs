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
    let mut is_visible = use_signal(|| true);
    rsx! {
        document::Title { "Stack offline verification" }
        document::Style { "{STYLES}" }
        document::Style { "@font-face {{font-family:Figtree;src:url('{FIGTREE}') format('woff2');font-style:normal;font-weight:300 900;font-display:swap;}}" }
        main { "data-astryx-theme": "astryx",
            button { id: "st-remove-patterns", onclick: move |_| is_visible.toggle(), "Toggle pattern mount" }
            if is_visible() { StackExamples {} }
        }
    }
}
