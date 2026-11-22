use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use crate::form_layout::form_layout_context::{FormLayoutContext, FormOptionality};
use crate::{Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue};

/// Arrangement of the radio controls within the group.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum RadioListOrientation {
    /// Stack controls in a column.
    #[default]
    Vertical,
    /// Arrange controls along the inline axis.
    Horizontal,
}

/// Named field containing caller-owned radio controls.
#[component]
pub fn RadioList(
    /// Visible name for the radio group.
    label: String,
    /// Radio controls sharing a caller-owned value and native name.
    children: Element,
    #[props(default)] description: Option<String>,
    #[props(default)] label_tooltip: Option<String>,
    #[props(default)] status: Option<FieldStatusInput>,
    #[props(default)] orientation: RadioListOrientation,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_optional: bool,
    /// Marks the group as required; native validation belongs to its controls.
    #[props(default)]
    is_required: bool,
    #[props(default)] width: Option<SizeValue>,
    /// Class on the outer field.
    #[props(default)]
    class: String,
    /// Inline declarations on the outer field.
    #[props(default)]
    style: String,
    /// Native attributes on the radio group.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| format!("astryx-radio-list-{}", current_scope_id().0));
    let label_id = format!("{id}-label");
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-desc"));
    let is_invalid = status
        .as_ref()
        .is_some_and(|status| status.status_type == InputStatusType::Error);
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
    let is_effectively_required = !is_optional
        && (is_required
            || try_consume_context::<FormLayoutContext>().is_some_and(|context| {
                context().default_optionality == Some(FormOptionality::Required)
            }));
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
    let orientation = match orientation {
        RadioListOrientation::Vertical => "vertical",
        RadioListOrientation::Horizontal => "horizontal",
    };
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "role"
                    | "aria-labelledby"
                    | "aria-describedby"
                    | "aria-invalid"
                    | "aria-required"
                    | "data-orientation"
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        Field { label, input_id: id, label_id, is_group_label: true, is_label_hidden, is_optional, is_required,
            description, description_id, status, label_tooltip, width, class, style, status_variant: FieldStatusPlacement::Detached,
            div { class: "astryx-radio-list", role: "radiogroup", "data-orientation": orientation,
                "aria-labelledby": labelled_by, "aria-describedby": described_by,
                "aria-invalid": is_invalid.then_some("true"), "aria-required": is_effectively_required.then_some("true"),
                ..attributes, {children}
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
