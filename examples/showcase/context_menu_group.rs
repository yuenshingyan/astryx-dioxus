use dioxus::prelude::*;

use astryx_dioxus::{ContextMenuDivider, ContextMenuGroup, ContextMenuItem};

#[component]
pub(super) fn ContextMenuGroupExamples() -> Element {
    rsx! {
        section { id: "context-menu-group-examples",
            h2 { "ContextMenuGroup" }
            p { "Inline group preview; no context-menu popup or keyboard controller." }
            div { role: "menu", "aria-label": "Document actions", style: "max-width:320px",
                ContextMenuGroup { title: rsx! { "Organize" },
                    ContextMenuItem { label: "Move" }
                    ContextMenuItem { label: "Duplicate" }
                }
                ContextMenuDivider {}
                ContextMenuGroup {
                    ContextMenuItem { label: "Copy link" }
                }
            }
        }
    }
}
