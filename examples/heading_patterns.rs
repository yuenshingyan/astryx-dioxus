//! Isolated offline Heading preview; serve on port 8086.
use astryx_dioxus::STYLES;
use dioxus::prelude::*;

#[path = "showcase/heading.rs"]
mod heading;
use heading::HeadingExamples;

const FIGTREE: Asset = asset!("/assets/fonts/Figtree-latin.woff2");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Style { "{STYLES}" }
        document::Style { "@font-face {{font-family:Figtree;src:url('{FIGTREE}') format('woff2');font-weight:300 900;}} body {{--font-family-heading:Figtree,sans-serif;font-family:Figtree,sans-serif;margin:0}}" }
        main { HeadingExamples {} }
    }
}
