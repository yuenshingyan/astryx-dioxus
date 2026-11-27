use dioxus::prelude::*;

use astryx_dioxus::{ContextMenu, ContextMenuItem};

#[component]
pub(super) fn ContextMenuExamples() -> Element {
    let mut action = use_signal(|| "No action selected");
    rsx! {
        section { id: "context-menu-examples",
            h2 { "Context Menu" }
            ContextMenu { label: "Editing actions",
                menu_content: rsx! {
                    for label in ["Cut", "Copy", "Paste"] {
                        ContextMenuItem { key: "{label}", label, has_close_on_select: label != "Paste", onclick: move |_| action.set(label) }
                    }
                },
                div { id: "context-menu-demo-area", style: "padding:48px;border:2px dashed var(--color-border-emphasized);border-radius:8px;text-align:center", "Right-click or press Shift+F10 in this area" }
            }
            output { "{action()}" }
        }
    }
}
