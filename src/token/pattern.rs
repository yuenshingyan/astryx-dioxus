// Adapted from Token/Token.tsx at the documented Astryx pin.
// Private exact-pattern slice: default/md, label only. Not a public Token API.
use dioxus::prelude::*;

#[component]
pub(crate) fn PatternToken(label: String) -> Element {
    rsx! {
        span { class: "astryx-token", "data-size": "md", "data-color": "default",
            span { class: "astryx-token__label", dir: "ltr", "{label}" }
        }
    }
}
