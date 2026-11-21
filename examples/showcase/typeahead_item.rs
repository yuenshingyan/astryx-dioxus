use dioxus::prelude::*;

use astryx_dioxus::{Avatar, TypeaheadItem, TypeaheadItemContent};

const PEOPLE: [(&str, &str); 5] = [
    ("Alice Johnson", "Engineer"),
    ("Bob Smith", "Designer"),
    ("Charlie Brown", "Product Manager"),
    ("Diana Prince", "Data Scientist"),
    ("Eve Davis", "QA Engineer"),
];

#[component]
pub(super) fn TypeaheadItemExamples() -> Element {
    rsx! {
        section { id: "typeahead-item-examples", "data-astryx-theme": "neutral",
            h2 { "TypeaheadItem" }
            p { "Search result previews without the parent typeahead." }
            h3 { "Basic" }
            div { id: "typeahead-item-basic", style: "max-width:320px;display:grid;gap:8px",
                for (label, description) in PEOPLE.into_iter().take(3) {
                    TypeaheadItem { key: "{label}", item: TypeaheadItemContent { label: label.into(), element: None }, description }
                }
            }
            h3 { "Showcase" }
            div { id: "typeahead-item-people", style: "max-width:320px;display:grid;gap:8px",
                for (label, description) in PEOPLE {
                    TypeaheadItem { key: "{label}", item: TypeaheadItemContent { label: label.into(), element: None },
                        description, icon: rsx! { Avatar { name: label, "aria-hidden": "true" } },
                    }
                }
            }
            h3 { "Narrow result" }
            TypeaheadItem { id: "typeahead-item-narrow", dir: "rtl", style: "width:120px", is_disabled: true,
                item: TypeaheadItemContent { label: "Charlie Brown — Product Manager".into(), element: None },
                description: "Product management and planning", icon: rsx! { Avatar { name: "Charlie Brown", "aria-hidden": "true" } },
            }
        }
    }
}
