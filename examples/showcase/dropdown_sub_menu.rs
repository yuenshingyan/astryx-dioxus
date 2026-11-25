use dioxus::prelude::*;

use astryx_dioxus::{DropdownMenu, DropdownMenuItem, DropdownMenuSubMenu};

#[component]
pub(super) fn DropdownSubMenuExamples() -> Element {
    let mut last_action = use_signal(String::new);
    rsx! {
        section { id: "dropdown-sub-menu-examples",
            h2 { "Dropdown Menu with Submenu" }
            DropdownMenu { label: "Actions",
                DropdownMenuItem { label: "Rename", onclick: move |_| last_action.set("Rename".to_owned()) }
                DropdownMenuSubMenu { label: "Move to",
                    for label in ["Projects", "Archive", "Trash"] {
                        DropdownMenuItem { key: "{label}", label, onclick: move |_| last_action.set(format!("Move to {label}")) }
                    }
                }
                DropdownMenuItem { label: "Delete", onclick: move |_| last_action.set("Delete".to_owned()) }
            }
            output { "{last_action()}" }
        }
    }
}
