use dioxus::prelude::*;

use crate::form_layout::form_layout_context::{FormLayoutContext, FormLayoutDirection};
use crate::{
    FieldStatus, FieldStatusVariant, InputStatusType, SizeValue, Text, TextDisplay, TextType,
    VisuallyHidden,
};

use super::field_label::FieldLabel;

/// Validation state and optional message associated with a field.
#[derive(Clone, PartialEq)]
pub struct FieldStatusInput {
    /// Validation severity.
    pub status_type: InputStatusType,
    /// Message displayed below the control.
    pub message: Option<String>,
    /// ID referenced by the control. Defaults to `<input_id>-status`.
    pub message_id: Option<String>,
}

/// Placement of the field's inline validation message.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum FieldStatusPlacement {
    /// Overlap an opaque, bordered, fixed-height control that reflects its size.
    /// Choose `Detached` for controls that cannot mask the overlap.
    #[default]
    Attached,
    /// Keep the message separate from the control.
    Detached,
}

/// Composes a label, caller-owned control, and optional validation message.
/// The caller wires the control's name and description references and owns its
/// required, disabled, and invalid states. `is_required` affects only the label.
#[component]
pub fn Field(
    label: String,
    input_id: String,
    children: Element,
    /// Label element ID for group naming.
    #[props(default)]
    label_id: Option<String>,
    #[props(default)] is_group_label: bool,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_optional: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] label_icon: Option<Element>,
    /// Short help revealed by a separate button next to the label.
    #[props(default)]
    label_tooltip: Option<String>,
    /// Localized accessible name for the help button.
    #[props(default)]
    label_tooltip_name: Option<String>,
    #[props(default)] description: Option<String>,
    /// ID referenced by the control. Defaults to `<input_id>-desc`.
    #[props(default)]
    description_id: Option<String>,
    #[props(default)] status: Option<FieldStatusInput>,
    #[props(default)] status_variant: FieldStatusPlacement,
    /// Width of the entire field outside horizontal-label layouts.
    #[props(default)]
    width: Option<SizeValue>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = div, extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let is_horizontal = try_consume_context::<FormLayoutContext>()
        .is_some_and(|context| context().direction == FormLayoutDirection::HorizontalLabels);
    let description = description.filter(|text| !text.is_empty());
    let description_id =
        description_id.or_else(|| description.as_ref().map(|_| format!("{input_id}-desc")));
    let (inline_description, inline_description_id) = if is_horizontal {
        (None, None)
    } else {
        (
            description.as_ref().map(|text| rsx! { "{text}" }),
            description_id.clone(),
        )
    };
    let label = rsx! { FieldLabel { label, input_id: input_id.clone(), label_id, is_group_label, is_label_hidden,
        is_optional, is_required, is_disabled, label_icon, label_tooltip, label_tooltip_name,
        description: inline_description, description_id: inline_description_id,
    } };
    let variant = match status_variant {
        FieldStatusPlacement::Attached => FieldStatusVariant::ATTACHED,
        FieldStatusPlacement::Detached => FieldStatusVariant::DETACHED,
    };
    let message = status.and_then(|status| {
        let message = status.message.filter(|message| !message.is_empty())?;
        let id = status
            .message_id
            .unwrap_or_else(|| format!("{input_id}-status"));
        Some(rsx! { FieldStatus { r#type: status.status_type, message, variant, id } })
    });
    let style = if is_horizontal {
        style
    } else {
        let width = width.and_then(|width| width.as_css().ok());
        width.map_or_else(|| style.clone(), |width| format!("width:{width};{style}"))
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "data-layout" | "data-label-hidden"))
        .collect();
    rsx! {
        div { class: "astryx-field {class}", style: (!style.is_empty()).then_some(style),
            "data-layout": is_horizontal.then_some("horizontal-labels"), "data-label-hidden": is_label_hidden.then_some("true"), ..attributes,
            if is_horizontal {
                div { class: "astryx-field__aligned-label", {label} }
                div { class: "astryx-field__control-status",
                    if let Some(description) = description {
                        if is_label_hidden { VisuallyHidden { id: description_id, "{description}" } }
                        else {
                            Text { text_type: TextType::SUPPORTING, display: TextDisplay::Block,
                                class: "astryx-field-label__description", id: description_id, "{description}"
                            }
                        }
                    }
                    {children}
                    {message}
                }
            } else {
                {label}
                if status_variant == FieldStatusPlacement::Attached {
                    div { class: "astryx-field__control-status", {children} {message} }
                } else { {children} {message} }
            }
        }
    }
}
