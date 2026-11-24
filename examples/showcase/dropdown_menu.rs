use dioxus::prelude::*;

use astryx_dioxus::{DropdownMenu, DropdownMenuItem};

#[component]
pub(super) fn DropdownMenuExamples() -> Element {
    let mut action = use_signal(|| "No action selected");
    rsx! {
        section { id: "dropdown-menu-examples",
            h2 { "Dropdown Menu" }
            DropdownMenu { label: "Actions",
                for label in ["Edit", "Duplicate", "Delete"] {
                    DropdownMenuItem { key: "{label}", label, is_disabled: label == "Duplicate", onclick: move |_| action.set(label) }
                }
            }
            output { "{action()}" }
            button { r#type: "button", id: "dropdown-menu-after", "After menu" }
        }
    }
}
