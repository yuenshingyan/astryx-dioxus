use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::field::input_clear_button::InputClearButton;
use crate::{Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue};

/// Native numeric field with browser-owned editing and constraint validation.
/// The change callback reports a committed number, or `None` for an empty DOM value.
#[component]
pub fn NumberInput(
    label: String,
    value: Option<f64>,
    #[props(default)] on_change: Option<EventHandler<Option<f64>>>,
    #[props(default)] min: Option<f64>,
    #[props(default)] max: Option<f64>,
    #[props(default)] step: Option<f64>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] description: Option<String>,
    /// Visible units that describe the input without changing its numeric value.
    #[props(default)]
    units: Option<String>,
    /// Caller-owned validation status, independent of native constraints.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Display the status attached to the input or below it as a separate message.
    #[props(default)]
    status_variant: FieldStatusPlacement,
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_read_only: bool,
    /// Show a clear action for populated, editable values.
    #[props(default)]
    has_clear: bool,
    /// Accessible name of the clear action. Defaults to `Clear {label}`.
    #[props(default)]
    clear_label: Option<String>,
    #[props(default)] is_label_hidden: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-number-input-{}", current_scope_id().0));
    if [value, min, max, step]
        .into_iter()
        .flatten()
        .any(|n| !n.is_finite())
        || min.zip(max).is_some_and(|(min, max)| min > max)
        || step.is_some_and(|step| step <= 0.0)
    {
        return VNode::empty();
    }
    let description_id = description
        .as_ref()
        .filter(|text| !text.is_empty())
        .map(|_| format!("{id}-description"));
    let severity = status.as_ref().map(|status| status.status_type);
    let status_id = status
        .as_ref()
        .filter(|status| {
            status
                .message
                .as_ref()
                .is_some_and(|message| !message.is_empty())
        })
        .map(|status| {
            status
                .message_id
                .clone()
                .unwrap_or_else(|| format!("{id}-status"))
        });
    let units = units.filter(|text| !text.trim().is_empty());
    let units_id = units.as_ref().map(|_| format!("{id}-units"));
    let described_by = [
        description_id.as_deref(),
        status_id.as_deref(),
        units_id.as_deref(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" ");
    let should_show_clear = has_clear && value.is_some() && !is_disabled && !is_read_only;
    let clear_label = clear_label
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| format!("Clear {label}"));
    let mut input_node = use_signal(|| None::<MountedEvent>);
    rsx! {
        Field { class: "astryx-number-input", label, input_id: id.clone(), is_label_hidden, is_disabled, is_required,
            width, description, description_id, status, status_variant,
            div { class: "astryx-number-input__entry", "data-units": units.is_some().then_some("true"),
                "data-has-clear": should_show_clear.then_some("true"),
                "data-status": severity.map(InputStatusType::as_str),
            input { class: "astryx-number-input__control", id, r#type: "number", name: html_name,
                value: value.map_or_else(String::new, |value| value.to_string()),
                min: min.map(|min| min.to_string()), max: max.map(|max| max.to_string()),
                step: step.map_or_else(|| "any".to_owned(), |step| step.to_string()),
                placeholder, required: is_required, readonly: is_read_only, disabled: is_disabled,
                "aria-describedby": (!described_by.is_empty()).then_some(described_by),
                "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                "data-status": severity.map(InputStatusType::as_str),
                onmounted: move |event| input_node.set(Some(event)),
                onkeydown: move |event: KeyboardEvent| {
                    if should_cancel_step(event.key(), event.modifiers(), event.is_composing(), !is_disabled && !is_read_only) {
                        event.prevent_default();
                    }
                },
                onchange: move |event| {
                    if !is_disabled && !is_read_only && let Some(handler) = on_change {
                        let raw = event.value();
                        if raw.is_empty() { handler.call(None); }
                        else if let Ok(number) = raw.parse::<f64>() && number.is_finite() {
                            handler.call(Some(number));
                        }
                    }
                },
            }
            if let Some(units) = units.as_ref() {
                span { id: units_id, class: "astryx-number-input__units", "{units}" }
            }
            if should_show_clear {
                InputClearButton { label: clear_label, onclick: move |_| {
                    let input = input_node.peek().clone();
                    if let Some(handler) = on_change { handler.call(None); }
                    if let Some(input) = input {
                        spawn(async move { let _ = input.data().set_focus(true).await; });
                    }
                } }
            }
            }
        }
    }
}

fn should_cancel_step(
    key: Key,
    modifiers: Modifiers,
    is_composing: bool,
    is_editable: bool,
) -> bool {
    // Unlike upstream's text input, a native number input can step by default.
    // Cancel only modified arrows; leave ordinary steps and commits to the browser.
    is_editable
        && !is_composing
        && matches!(key, Key::ArrowUp | Key::ArrowDown)
        && modifiers
            .intersects(Modifiers::ALT | Modifiers::CONTROL | Modifiers::META | Modifiers::SHIFT)
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;

    use super::should_cancel_step;

    #[test]
    fn cancels_modified_vertical_arrows_in_editable_fields() {
        for key in [Key::ArrowUp, Key::ArrowDown] {
            for modifier in [
                Modifiers::ALT,
                Modifiers::CONTROL,
                Modifiers::META,
                Modifiers::SHIFT,
            ] {
                assert!(
                    should_cancel_step(key.clone(), modifier, false, true),
                    "{key:?} with {modifier:?}"
                );
            }
        }
    }

    #[test]
    fn preserves_native_defaults_outside_modified_editable_steps() {
        for (key, modifiers, is_composing, is_editable) in [
            (Key::ArrowUp, Modifiers::empty(), false, true),
            (Key::ArrowDown, Modifiers::empty(), false, true),
            (Key::Enter, Modifiers::SHIFT, false, true),
            (Key::ArrowLeft, Modifiers::SHIFT, false, true),
            (Key::ArrowDown, Modifiers::SHIFT, true, true),
            (Key::ArrowUp, Modifiers::SHIFT, false, false),
        ] {
            assert!(
                !should_cancel_step(key.clone(), modifiers, is_composing, is_editable),
                "{key:?} with {modifiers:?}"
            );
        }
    }
}
