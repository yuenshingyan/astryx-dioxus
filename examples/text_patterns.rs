//! Isolated offline Text fixture / interaction preview (port 8085).
use dioxus::prelude::*;

use astryx_dioxus::{STYLES, Text};

#[path = "showcase/text.rs"]
mod text;

use text::TextExamples;

const FIGTREE: Asset = asset!("/assets/fonts/Figtree-latin.woff2");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_wide = use_signal(|| false);
    let mut has_caller_tabindex = use_signal(|| false);
    rsx! {
        document::Style { "{STYLES}" }
        document::Style { "@font-face {{font-family:Figtree;src:url('{FIGTREE}') format('woff2');font-weight:300 900;}} body {{font-family:Figtree,sans-serif;margin:0}}" }
        main {
            TextExamples {}
            section { id: "keyboard-overflow", "aria-label": "Keyboard overflow probes",
                button { id: "kb-before", "Start keyboard checks" }
                Text { id: "kb-fit", max_lines: 1, style: "width:100px", "Short" }
                Text { id: "kb-cut", max_lines: 1,
                    tabindex: has_caller_tabindex().then_some(-1),
                    style: if is_wide() { "width:800px" } else { "width:80px" },
                    "Full truncated text available to keyboard users"
                }
                Text { id: "kb-disabled", max_lines: 1, has_truncate_tooltip: false,
                    style: "width:80px", "Clipped text with tooltip explicitly disabled"
                }
                Text { id: "kb-rich", max_lines: 1, style: "width:120px",
                    a { id: "kb-link", href: "#keyboard-overflow", "Link" }
                    " followed by a long text continuation"
                }
                a { id: "kb-parent", href: "#keyboard-overflow",
                    Text { id: "kb-in-link", max_lines: 1, style: "width:80px",
                        "Long text inside an existing interactive control"
                    }
                }
                button { id: "kb-resize", onclick: move |_| is_wide.toggle(), "Resize text" }
                button { id: "kb-caller", onclick: move |_| has_caller_tabindex.toggle(), "Toggle caller tabindex" }
            }
        }
    }
}
