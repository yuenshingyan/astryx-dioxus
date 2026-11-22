use dioxus::prelude::*;

use astryx_dioxus::{Button, ChatComposerDrawer, Token};

#[component]
pub(super) fn ChatComposerDrawerExamples() -> Element {
    rsx! {
        section { id: "chat-composer-drawer-examples",
            h2 { "ChatComposerDrawer" }
            div { style: "padding-block-end:var(--radius-chat)",
                ChatComposerDrawer { count: 6, label: "Files",
                    for file in ["design-spec.pdf", "api-schema.json", "screenshot.png", "meeting-notes.md", "test-results.csv", "deploy-log.txt"] {
                        Token { key: "{file}", label: file }
                    }
                    Button { label: "Review files" }
                }
            }
            div { style: "padding-block-end:var(--radius-chat)",
                ChatComposerDrawer { Token { label: "notes.txt" } }
            }
        }
    }
}
