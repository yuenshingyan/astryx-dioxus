use dioxus::prelude::*;

use astryx_dioxus::{Tab, TabList, TabMenu, TabMenuOption};

#[component]
pub(super) fn TabMenuExamples() -> Element {
    let mut selected = use_signal(|| "settings".to_owned());
    let options = [
        ("settings", "Settings"),
        ("integrations", "Integrations"),
        ("billing", "Billing"),
    ]
    .into_iter()
    .map(|(value, label)| TabMenuOption {
        value: value.into(),
        label: label.into(),
        icon: None,
    })
    .collect();
    rsx! {
        section { id: "tab-menu-examples",
            h2 { "Tab Menu" }
            TabList { value: selected(), on_change: move |next| selected.set(next), label: "Workspace",
                Tab { value: "overview", label: "Overview" }
                Tab { value: "activity", label: "Activity" }
                TabMenu { label: "More", options }
            }
            button { id: "tab-menu-after", r#type: "button", "After tabs" }
        }
    }
}
