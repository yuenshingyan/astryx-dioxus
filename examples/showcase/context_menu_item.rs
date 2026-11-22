use dioxus::prelude::*;

use astryx_dioxus::{ContextMenuItem, DropdownMenuItemVariant};

#[component]
pub(super) fn ContextMenuItemExamples() -> Element {
    let mut action = use_signal(|| "No action yet");
    rsx! {
        section { id: "context-menu-item-examples",
            h2 { "ContextMenuItem" }
            p { "Inline action rows; no context-menu trigger or keyboard controller." }
            div { role: "menu", "aria-label": "Item actions", style: "max-width:320px",
                ContextMenuItem { label: "Edit", description: Some("Modify this item".into()), onclick: move |_| action.set("Edit requested") }
                ContextMenuItem { label: "Duplicate", description: Some("Create a copy".into()), onclick: move |_| action.set("Duplicate requested") }
                ContextMenuItem { label: "Delete", description: Some("This action cannot be undone".into()), variant: DropdownMenuItemVariant::Destructive,
                    onclick: move |_| action.set("Delete requested"),
                }
            }
            output { id: "context-menu-item-result", "{action}" }
        }
    }
}
