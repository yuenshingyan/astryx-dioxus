use dioxus::prelude::*;

use astryx_dioxus::{
    ButtonGroupOrientation, ButtonSize, ToggleButton, ToggleButtonGroup, ToggleButtonGroupSelection,
};

#[component]
pub(super) fn ToggleButtonGroupExamples() -> Element {
    let mut view = use_signal(|| Some("grid".to_string()));
    let mut formats = use_signal(|| vec!["bold".to_string()]);
    let single = ToggleButtonGroupSelection::Single {
        value: view(),
        onchange: EventHandler::new(move |next| view.set(next)),
    };
    let multiple = ToggleButtonGroupSelection::Multiple {
        values: formats(),
        onchange: EventHandler::new(move |next| formats.set(next)),
    };
    rsx! {
        section { id: "toggle-button-group-examples", "data-astryx-theme": "neutral",
            h2 { "ToggleButtonGroup" }
            h3 { "Single selection" }
            ToggleButtonGroup { label: "View mode", id: "toggle-view", selection: single, size: ButtonSize::Small,
                ToggleButton { value: "list", label: "List view", id: "toggle-list", is_icon_only: true, icon: rsx! { span { "≡" } } }
                ToggleButton { value: "grid", label: "Grid view", id: "toggle-grid", is_icon_only: true, icon: rsx! { span { "▦" } } }
                ToggleButton { value: "table", label: "Table view", id: "toggle-table", is_icon_only: true, icon: rsx! { span { "▤" } }, size: ButtonSize::Medium }
            }
            h3 { "Multiple selection" }
            ToggleButtonGroup { label: "Text formatting", id: "toggle-formats", selection: multiple,
                ToggleButton { value: "bold", label: "Bold", id: "toggle-bold" }
                ToggleButton { value: "italic", label: "Italic", id: "toggle-italic" }
                ToggleButton { value: "underline", label: "Underline" }
                ToggleButton { value: "strikethrough", label: "Strikethrough" }
            }
            h3 { "Disabled vertical group" }
            ToggleButtonGroup { label: "Unavailable views", id: "toggle-disabled-group", orientation: ButtonGroupOrientation::Vertical, is_disabled: true,
                selection: ToggleButtonGroupSelection::Single { value: None, onchange: EventHandler::new(|_| {}) },
                ToggleButton { value: "table", label: "Unavailable table" }
                ToggleButton { label: "Unavailable standalone" }
            }
        }
    }
}
