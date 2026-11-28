use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::radio_list_context::{RadioListContext, RadioListSettings};
use crate::form_layout::form_layout_context::{FormLayoutContext, FormOptionality};
use crate::{
    Field, FieldStatusInput, FieldStatusPlacement, IndicatorSize, InputStatusType, SizeValue,
};

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
    /// Selected value for RadioListItem descendants.
    #[props(default)]
    value: Option<String>,
    /// Receives selection requests from RadioListItem descendants.
    #[props(default)]
    on_change: Option<EventHandler<String>>,
    /// Native form name shared by RadioListItem descendants.
    #[props(default)]
    html_name: Option<String>,
    /// Disables RadioListItem descendants and overrides caller `aria-disabled` on the group.
    #[props(default)]
    is_disabled: bool,
    #[props(default = IndicatorSize::Medium)] size: IndicatorSize,
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
    let settings = RadioListSettings {
        name: html_name.unwrap_or_else(|| id.clone()),
        value,
        on_change,
        size,
        is_disabled,
        is_required,
    };
    let mut context = use_context_provider(|| RadioListContext(Signal::new(settings.clone())));
    if *context.0.peek() != settings {
        context.0.set(settings);
    }
    let group_id = id.clone();
    let mount_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
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
    // The caller's ARIA state describes the group; only is_disabled changes the controls.
    let aria_disabled = if is_disabled {
        Some("true".to_owned())
    } else {
        attributes
            .iter()
            .filter(|attr| attr.name == "aria-disabled")
            .find_map(|attr| match &attr.value {
                AttributeValue::Text(text) => Some(text.clone()),
                AttributeValue::Bool(value) => Some(value.to_string()),
                _ => None,
            })
    };
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
                    | "data-astryx-radio-list"
                    | "onmounted"
                    | "aria-disabled"
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        Field { label, input_id: id, label_id, is_group_label: true, is_label_hidden, is_optional, is_required, is_disabled,
            description, description_id, status, label_tooltip, width, class, style, status_variant: FieldStatusPlacement::Detached,
            div { class: "astryx-radio-list", role: "radiogroup", "data-orientation": orientation,
                "aria-labelledby": labelled_by, "aria-describedby": described_by,
                "aria-invalid": is_invalid.then_some("true"), "aria-required": is_effectively_required.then_some("true"),
                "aria-disabled": aria_disabled, "data-astryx-radio-list": group_id,
                onmounted: move |_| {
                    if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                    let eval = document::eval(include_str!("sync_radios.js"));
                    let _ = eval.send(mount_id.clone());
                    controller.set(Some(eval));
                },
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
