//! Native label and description adapted from Astryx Field/FieldLabel.tsx.

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::InfoTip;
use crate::form_layout::FormOptionality;
use crate::form_layout::form_layout_context::FormLayoutContext;
use crate::hooks::use_input_container::use_input_container;
use crate::utils::is_renderable::is_renderable;

/// Labels an individual control or provides an ID-addressable name for a group.
#[component]
pub fn FieldLabel(
    /// Text of the label.
    label: String,
    /// ID of the associated control.
    input_id: String,
    /// ID of the label itself.
    #[props(default)]
    label_id: Option<String>,
    /// Render a span rather than a native label for a group of controls.
    #[props(default)]
    is_group_label: bool,
    /// Visually hide the label and description without removing their semantics.
    #[props(default)]
    is_label_hidden: bool,
    /// Apply disabled styling and suppress description activation.
    #[props(default)]
    is_disabled: bool,
    /// Mark this field optional when it differs from the layout default.
    #[props(default)]
    is_optional: bool,
    /// Mark this field required when it differs from the layout default.
    #[props(default)]
    is_required: bool,
    /// Leading artwork.
    #[props(default)]
    label_icon: Option<Element>,
    /// Short help revealed by a separate InfoTip button.
    #[props(default)]
    label_tooltip: Option<String>,
    /// Localized name of the help button; defaults to “More information about {label}”.
    #[props(default)]
    label_tooltip_name: Option<String>,
    /// Supporting content, kept separate from the control's accessible name.
    #[props(default)]
    description: Option<Element>,
    /// ID for the control's `aria-describedby` association.
    #[props(default)]
    description_id: Option<String>,
    /// Localized optional marker.
    #[props(default = "Optional".into())]
    optional_text: String,
    /// Localized required marker.
    #[props(default = "Required".into())]
    required_text: String,
    /// Caller classes on the label.
    #[props(default)]
    class: String,
    /// Inline declarations on the label.
    #[props(default)]
    style: String,
    /// Native attributes and callbacks on the label.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let default = try_consume_context::<FormLayoutContext>()
        .and_then(|context| context().default_optionality);
    let marker = if is_optional && default != Some(FormOptionality::Optional) {
        Some(optional_text)
    } else if is_required && default != Some(FormOptionality::Required) {
        Some(required_text)
    } else {
        None
    };
    let id = format!("field-label-{}", current_scope_id().0);
    let can_forward = !is_group_label && !is_disabled && !is_label_hidden && !input_id.is_empty();
    let mount_description = use_input_container(id.clone(), can_forward.then(|| input_id.clone()));
    let classes = format!("astryx-field-label {class}");
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class" | "style" | "for" | "data-label-hidden" | "data-disabled"
            ) && !(attr.name == "id" && label_id.is_some())
        })
        .collect();
    let help_name = label_tooltip_name.unwrap_or_else(|| format!("More information about {label}"));
    let content = rsx! {
        if let Some(icon) = label_icon.filter(is_renderable) { {icon} }
        {label}
        if let Some(marker) = marker { span { class: "astryx-field-label__marker", span { "aria-hidden": "true", " ∙ " } {marker} } }
    };
    let label = if is_group_label {
        rsx! { span { class: classes, id: label_id, style: (!style.is_empty()).then_some(style),
        "data-label-hidden": is_label_hidden.then_some("true"), "data-disabled": is_disabled.then_some("true"), ..attributes, {content} } }
    } else {
        rsx! { label { class: classes, id: label_id, r#for: input_id, style: (!style.is_empty()).then_some(style),
        "data-label-hidden": is_label_hidden.then_some("true"), "data-disabled": is_disabled.then_some("true"), ..attributes, {content} } }
    };
    let label_with_help = if let Some(help) = label_tooltip.filter(|help| !help.trim().is_empty()) {
        let help_class = if is_label_hidden {
            "astryx-field-label__help--hidden"
        } else {
            ""
        };
        rsx! {
            span { class: "astryx-field-label__row", "data-label-hidden": is_label_hidden.then_some("true"),
                {label}
                InfoTip { content: help, label: help_name, class: help_class }
            }
        }
    } else {
        label
    };
    let Some(description) = description.filter(is_renderable) else {
        return label_with_help;
    };
    rsx! {
        div { class: "astryx-field-label__group", "data-label-hidden": is_label_hidden.then_some("true"),
            {label_with_help}
            span { class: "astryx-field-label__description", id: description_id,
                "data-label-hidden": is_label_hidden.then_some("true"), "data-forwards-click": can_forward.then_some("true"),
                "data-astryx-input-description": id, onmounted: mount_description, {description} }
        }
    }
}
