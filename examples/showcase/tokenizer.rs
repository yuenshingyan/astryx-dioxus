use dioxus::prelude::*;

use astryx_dioxus::{
    BaseTypeaheadOption, FieldStatusInput, FieldStatusPlacement, InputStatusType, Tokenizer,
    TokenizerChange,
};

#[component]
pub(super) fn TokenizerExamples() -> Element {
    let options: Vec<_> = [
        "Design",
        "Engineering",
        "Marketing",
        "Product",
        "Research",
        "Sales",
    ]
    .into_iter()
    .enumerate()
    .map(|(index, label)| BaseTypeaheadOption {
        id: (index + 1).to_string(),
        label: label.into(),
        description: None,
    })
    .collect();
    let mut value = use_signal(|| options[..2].to_vec());
    let status = (value.read().len() < 2).then(|| FieldStatusInput {
        status_type: InputStatusType::Error,
        message: Some("Select at least two tags.".into()),
        message_id: None,
    });
    rsx! {
        section { id: "tokenizer-examples",
            h2 { "Tokenizer" }
            form { onsubmit: move |event| event.prevent_default(),
                Tokenizer { label: "Tags", options, value: value(), placeholder: "Search...", html_name: "tags",
                    width: Some(400.into()), has_entries_on_focus: true,
                    description: "Choose tags for the project.", status, status_variant: FieldStatusPlacement::Detached,
                    on_change: move |(next, _): (Vec<BaseTypeaheadOption>, TokenizerChange)| value.set(next),
                }
            }
        }
    }
}
