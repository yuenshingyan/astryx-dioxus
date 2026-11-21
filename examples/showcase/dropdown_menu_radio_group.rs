use dioxus::prelude::*;

use astryx_dioxus::DropdownMenuRadioGroup;

#[component]
pub(super) fn DropdownMenuRadioGroupExamples() -> Element {
    let mut sort = use_signal(|| "newest".to_owned());
    rsx! {
        section { id: "dropdown-menu-radio-group-examples",
            h2 { "DropdownMenuRadioGroup" }
            p { "Native radios stand in for menu items in this inline preview." }
            h3 { id: "sort-options-heading", "Sort options" }
            DropdownMenuRadioGroup {
                id: "sort-options", label: "Sort by", "aria-labelledby": "sort-options-heading",
                value: sort(), on_change: move |value| sort.set(value),
                for (value, label) in [("newest", "Newest first"), ("oldest", "Oldest first"), ("name", "Name")] {
                    label { key: "{value}",
                        input { r#type: "radio", name: "dropdown-sort-preview", value,
                            checked: sort.read().as_str() == value, onchange: move |_| sort.set(value.to_owned()),
                        }
                        " {label}"
                    }
                }
            }
        }
    }
}
