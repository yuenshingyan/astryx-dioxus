use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{ElementSize, Field, FieldStatusInput, FieldStatusPlacement, SizeProvider, use_size};

use super::input_group_internal_context::{
    InputGroupInternalContext, InputGroupInternalState, use_input_group_internal,
};

/// Composes a labeled row of input controls and addons. Native controls remain
/// caller-owned, including their own label and disabled state.
#[component]
pub fn InputGroup(
    /// Accessible name for the group.
    label: String,
    /// Input controls and addons in logical order.
    children: Element,
    /// Optional description for the group.
    #[props(default)]
    description: Option<String>,
    /// Validation state and optional detached message.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Hide the label visually without removing its name.
    #[props(default)]
    is_label_hidden: bool,
    /// Marks the label as optional when it differs from the layout default.
    #[props(default)]
    is_optional: bool,
    /// Marks the label as required when it differs from the layout default.
    #[props(default)]
    is_required: bool,
    /// Disable the group and its compatible descendants.
    #[props(default)]
    is_disabled: bool,
    /// Override the inherited element size.
    #[props(default)]
    size: Option<ElementSize>,
    /// Additional row classes.
    #[props(default)]
    class: String,
    /// Inline row styles.
    #[props(default)]
    style: Option<String>,
    /// Native row attributes. The group owns its accessible name and state.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let size = use_size(size);
    let is_disabled = is_disabled || use_input_group_internal().is_disabled;
    let state = InputGroupInternalState { is_disabled };
    let InputGroupInternalContext(mut context) =
        use_context_provider(|| InputGroupInternalContext(Signal::new(state)));
    if *context.peek() != state {
        context.set(state);
    }

    let input_id = format!("astryx-input-group-{}", current_scope_id().0);
    let label_id = format!("{input_id}-label");
    let description_id = description
        .as_ref()
        .filter(|text| !text.is_empty())
        .map(|_| format!("{input_id}-desc"));
    let status = status.map(|mut status| {
        if status.message.as_ref().is_some_and(|text| !text.is_empty()) {
            status
                .message_id
                .get_or_insert_with(|| format!("{input_id}-status"));
        } else {
            status.message_id = None;
        }
        status
    });
    let status_type = status.as_ref().map(|status| status.status_type.as_str());
    let described_by = [
        description_id.as_deref(),
        status
            .as_ref()
            .and_then(|status| status.message_id.as_deref()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" ");
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "role"
                    | "aria-label"
                    | "aria-labelledby"
                    | "aria-describedby"
                    | "aria-disabled"
                    | "data-size"
                    | "data-status"
            )
        })
        .collect();
    rsx! {
        SizeProvider { size,
            Field { label, input_id, label_id: label_id.clone(), is_group_label: true, is_label_hidden,
                description, description_id, is_optional, is_required, is_disabled, status, status_variant: FieldStatusPlacement::Detached,
                div { class: "astryx-input-group {class}", style, role: "group", "aria-labelledby": label_id,
                    "aria-describedby": (!described_by.is_empty()).then_some(described_by), "aria-disabled": is_disabled.then_some("true"),
                    "data-size": size.as_str(), "data-status": status_type, ..attributes, {children}
                }
            }
        }
    }
}
