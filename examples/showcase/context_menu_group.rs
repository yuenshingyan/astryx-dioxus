use dioxus::prelude::*;

use astryx_dioxus::{
    ContextMenuCheckboxItem, ContextMenuDivider, ContextMenuGroup, ContextMenuItem,
};

#[component]
pub(super) fn ContextMenuGroupExamples() -> Element {
    let mut show_details = use_signal(|| false);
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
                    ContextMenuCheckboxItem { id: "context-menu-checkbox-details", label: "Show details",
                        value: show_details(), on_change: move |checked| show_details.set(checked),
                    }
                }
            }
        }
    }
}
