use astryx_dioxus::STYLES;
use dioxus::prelude::*;

#[path = "showcase/aspect_ratio.rs"]
mod aspect_ratio;
use aspect_ratio::AspectRatioExamples;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Style { "{STYLES}" }
        main { "data-astryx-theme":"neutral", AspectRatioExamples {} }
    }
}
