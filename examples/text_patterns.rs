//! Isolated offline Text fixture / interaction preview (port 8085).
use astryx_dioxus::STYLES;
use dioxus::prelude::*;

#[path = "showcase/text.rs"]
mod text;

use text::TextExamples;

const FIGTREE: Asset = asset!("/assets/fonts/Figtree-latin.woff2");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Style { "{STYLES}" }
        document::Style { "@font-face {{font-family:Figtree;src:url('{FIGTREE}') format('woff2');font-weight:300 900;}} body {{font-family:Figtree,sans-serif;margin:0}}" }
        main { TextExamples {} }
    }
}
