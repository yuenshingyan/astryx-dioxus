use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use crate::{Field, FieldStatusInput, FieldStatusPlacement, List, ListDensity, SizeValue};

/// A named checkbox group with caller-owned rows and selection.
#[component]
pub fn CheckboxList(
    label: String,
    /// List rows containing checkbox controls.
    children: Element,
    #[props(default)] description: Option<String>,
    #[props(default)] status: Option<FieldStatusInput>,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_optional: bool,
    #[props(default)] density: ListDensity,
    #[props(default)] has_dividers: bool,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    /// Native attributes for the group, not its enclosing field.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| format!("astryx-checkbox-list-{}", current_scope_id().0));
    let label_id = format!("{id}-label");
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-desc"));
    let status = status.map(|mut status| {
        if status
            .message
            .as_ref()
            .is_some_and(|message| !message.is_empty())
        {
            status
                .message_id
                .get_or_insert_with(|| format!("{id}-status"));
        } else {
            status.message_id = None;
        }
        status
    });
    let labelled_by = join_ids([
        Some(label_id.as_str()),
        attribute_text(&attributes, "aria-labelledby"),
    ]);
    let described_by = join_ids([
        attribute_text(&attributes, "aria-describedby"),
        description_id.as_deref(),
        status
            .as_ref()
            .and_then(|status| status.message_id.as_deref()),
    ]);
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class" | "style" | "role" | "aria-labelledby" | "aria-describedby"
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        Field { label, input_id: id, label_id, is_group_label: true, is_label_hidden, is_optional,
            description, description_id, status, status_variant: FieldStatusPlacement::Detached,
            width, class: format!("astryx-checkbox-list {class}"), style,
            div { class: "astryx-checkbox-list__group", role: "group", "aria-labelledby": labelled_by,
                "aria-describedby": described_by, ..attributes,
                List { density, has_dividers, {children} }
            }
        }
    }
}

fn attribute_text<'a>(attributes: &'a [Attribute], name: &str) -> Option<&'a str> {
    attributes.iter().find_map(|attr| match &attr.value {
        AttributeValue::Text(text) if attr.name == name => Some(text.as_str()),
        _ => None,
    })
}

fn join_ids<'a>(ids: impl IntoIterator<Item = Option<&'a str>>) -> Option<String> {
    let ids = ids
        .into_iter()
        .flatten()
        .flat_map(str::split_whitespace)
        .collect::<Vec<_>>()
        .join(" ");
    (!ids.is_empty()).then_some(ids)
}
