use dioxus::prelude::*;

use astryx_dioxus::{
    PowerSearch, PowerSearchChoice, PowerSearchField, PowerSearchFilter, PowerSearchOperator,
    PowerSearchValue, PowerSearchValueKind,
};

#[component]
pub(super) fn PowerSearchExamples() -> Element {
    let mut filters = use_signal(|| {
        vec![
            PowerSearchFilter {
                field: "status".into(),
                operator: "is".into(),
                value: PowerSearchValue::Enum("open".into()),
            },
            PowerSearchFilter {
                field: "title".into(),
                operator: "contains".into(),
                value: PowerSearchValue::Text("dashboard".into()),
            },
        ]
    });
    let fields = vec![
        PowerSearchField {
            key: "status".into(),
            label: "Status".into(),
            operators: vec![PowerSearchOperator {
                key: "is".into(),
                label: "is".into(),
                kind: PowerSearchValueKind::Enum(
                    [
                        ("open", "Open"),
                        ("in_progress", "In Progress"),
                        ("closed", "Closed"),
                    ]
                    .into_iter()
                    .map(|(value, label)| PowerSearchChoice {
                        value: value.into(),
                        label: label.into(),
                    })
                    .collect(),
                ),
            }],
        },
        PowerSearchField {
            key: "title".into(),
            label: "Title".into(),
            operators: vec![PowerSearchOperator {
                key: "contains".into(),
                label: "contains".into(),
                kind: PowerSearchValueKind::Text,
            }],
        },
    ];
    rsx! {
        section { id: "power-search-examples",
            h2 { "Power Search" }
            div { style: "max-inline-size:400px",
                PowerSearch { label: "Search filters", fields, filters: filters(),
                    on_change: move |next| filters.set(next), placeholder: "Search by status, title...",
                }
            }
        }
    }
}
