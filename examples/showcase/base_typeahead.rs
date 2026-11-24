use dioxus::prelude::*;

use astryx_dioxus::{BaseTypeahead, BaseTypeaheadOption};

#[component]
pub(super) fn BaseTypeaheadExamples() -> Element {
    let mut value = use_signal(|| None::<BaseTypeaheadOption>);
    let options = [
        ("react", "React", "UI library"),
        ("remix", "Remix", "Web framework"),
        ("next", "Next.js", "Web framework"),
    ]
    .into_iter()
    .map(|(id, label, description)| BaseTypeaheadOption {
        id: id.into(),
        label: label.into(),
        description: Some(description.into()),
    })
    .collect();
    rsx! {
        section { id: "base-typeahead-examples",
            h2 { "Base Typeahead" }
            div { style: "max-inline-size:360px",
                BaseTypeahead { label: "Search frameworks", placeholder: "Search frameworks…",
                    options, value: value(), on_change: move |next| value.set(next), has_entries_on_focus: true }
            }
        }
    }
}
