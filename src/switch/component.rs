use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{
    FieldLabel, FieldStatus, FieldStatusInput, FieldStatusVariant, IndicatorSize, InputStatusType,
    SizeValue, Spinner, SpinnerSize,
};

/// Logical position of a switch's label.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SwitchLabelPosition {
    Start,
    #[default]
    End,
}

/// Spacing between the label and control.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SwitchLabelSpacing {
    #[default]
    Hug,
    Spread,
}

/// A controlled native on/off switch.
#[component]
pub fn Switch(
    label: String,
    value: bool,
    #[props(default)] on_change: Option<EventHandler<bool>>,
    #[props(default)] id: Option<String>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] label_tooltip: Option<String>,
    #[props(default)] status: Option<FieldStatusInput>,
    #[props(default)] is_disabled: bool,
    /// Keep the current value and block activation while a caller-owned operation runs.
    #[props(default)]
    is_loading: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_optional: bool,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] size: IndicatorSize,
    #[props(default)] label_position: SwitchLabelPosition,
    #[props(default)] label_spacing: SwitchLabelSpacing,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let owner = use_hook(|| format!("astryx-switch-{}", current_scope_id().0));
    let id = id.unwrap_or_else(|| owner.clone());
    let label_input_id = id.clone();
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-desc"));
    let is_error = status
        .as_ref()
        .is_some_and(|status| status.status_type == InputStatusType::Error);
    let status = status.and_then(|status| {
        Some((
            status.status_type,
            status.message.filter(|text| !text.is_empty())?,
            status.message_id.unwrap_or_else(|| format!("{id}-status")),
        ))
    });
    let descriptions = [
        description_id.as_deref(),
        status.as_ref().map(|(_, _, id)| id.as_str()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" ");
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let size = if size == IndicatorSize::Small {
        "sm"
    } else {
        "md"
    };
    let position = if label_position == SwitchLabelPosition::Start {
        "start"
    } else {
        "end"
    };
    rsx! {
        div { class: "astryx-switch-field", style: width.and_then(|width| width.as_css().ok()).map(|width| format!("width:{width}")),
            div { class: "astryx-switch-field__row", "data-position": position,
                "data-spread": (label_spacing == SwitchLabelSpacing::Spread && !is_label_hidden).then_some("true"),
                "data-label-hidden": is_label_hidden.then_some("true"),
                span { class: "astryx-switch-control", "data-size": size,
                    input { id: id.clone(), r#type: "checkbox", role: "switch", checked: value,
                        name: if is_disabled { None } else { html_name },
                        form: is_disabled.then(String::new), required: is_required,
                        "aria-disabled": is_disabled.then_some("true"), "aria-required": is_required.then_some("true"),
                        "aria-invalid": (is_error || is_required).then(|| is_error.to_string()),
                        "aria-busy": is_loading.then_some("true"),
                        "aria-describedby": (!descriptions.is_empty()).then_some(descriptions),
                        "data-astryx-switch-owner": owner.clone(), "data-checked": value.to_string(),
                        onmounted: move |_| {
                            if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                            let eval = document::eval(include_str!("sync_switch.js"));
                            let _ = eval.send(owner.clone());
                            controller.set(Some(eval));
                        },
                        onclick: move |event| { if is_disabled || is_loading { event.prevent_default(); } },
                        onchange: move |event| {
                            if !is_disabled && !is_loading && let Some(handler) = on_change { handler.call(event.checked()); }
                        },
                    }
                    span { class: "astryx-switch", "aria-hidden": "true",
                        span { class: "astryx-switch-thumb",
                            if is_loading { Spinner { size: SpinnerSize::Small, is_decorative: true } }
                        }
                    }
                }
                FieldLabel { class: "astryx-switch-label", label, input_id: label_input_id, label_tooltip,
                    is_label_hidden, is_disabled, is_optional, is_required,
                    description: description.map(|text| rsx! { "{text}" }), description_id,
                }
            }
            if let Some((severity, message, id)) = status {
                FieldStatus { r#type: severity, message, id, variant: FieldStatusVariant::DETACHED }
            }
        }
    }
}
