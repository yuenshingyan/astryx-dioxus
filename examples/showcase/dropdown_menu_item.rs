use dioxus::prelude::*;

use astryx_dioxus::{DropdownMenuItem, DropdownMenuItemVariant};

#[component]
pub(super) fn DropdownMenuItemExamples() -> Element {
    let mut last_action = use_signal(|| "No action yet");
    rsx! {
        section { id: "dropdown-menu-item-examples",
            h2 { "DropdownMenuItem" }
            p { "Inline action-row preview. Popup and keyboard navigation are not wired." }
            div { role: "menu", "aria-label": "Project actions", style: "max-width:320px",
                DropdownMenuItem { id: "menu-edit-project", label: "Edit project", onclick: move |_| last_action.set("Edit requested") }
                DropdownMenuItem { label: "Duplicate project", description: Some("Create an editable copy".into()), onclick: move |_| last_action.set("Duplicate requested") }
                DropdownMenuItem { id: "menu-delete-project", label: "Delete project", variant: DropdownMenuItemVariant::Destructive,
                    onclick: move |_| last_action.set("Delete requested"),
                }
                DropdownMenuItem { id: "menu-archive-project", label: "Archive unavailable", is_disabled: true,
                    onclick: move |_| last_action.set("Archive requested"),
                }
            }
            p { id: "project-menu-result", "{last_action}" }
        }
    }
}
