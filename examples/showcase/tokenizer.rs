use dioxus::prelude::*;

use astryx_dioxus::{
    BaseTypeaheadOption, FieldStatusInput, FieldStatusPlacement, InputStatusType, TokenizerChange,
    TokenizerWithClear,
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
    let mut change_count = use_signal(|| 0);
    let mut last_change = use_signal(String::new);
    let status = (value.read().len() < 2).then(|| FieldStatusInput {
        status_type: InputStatusType::Error,
        message: Some("Select at least two tags.".into()),
        message_id: None,
    });
    rsx! {
        section { id: "tokenizer-examples",
            h2 { "Tokenizer" }
            form { onsubmit: move |event| event.prevent_default(),
                TokenizerWithClear { label: "Tags", options, value: value(), placeholder: "Search...", html_name: "tags",
                    width: Some(400.into()), has_entries_on_focus: true,
                    description: "Choose tags for the project.", status, status_variant: FieldStatusPlacement::Detached,
                    on_change: move |(next, change): (Vec<BaseTypeaheadOption>, TokenizerChange)| {
                        value.set(next);
                        change_count += 1;
                        last_change.set(match change {
                            TokenizerChange::Add(item) => format!("Added {}", item.label),
                            TokenizerChange::Remove(item) => format!("Removed {}", item.label),
                        });
                    },
                }
                if change_count() > 0 {
                    p { id: "tokenizer-last-change", "Change {change_count}: {last_change}" }
                }
            }
        }
    }
}
