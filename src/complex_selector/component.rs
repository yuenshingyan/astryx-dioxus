use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::popover::use_popover;
use crate::{Field, LayerPlacement, SizeValue};

/// Bindings for caller-rendered selector content.
#[derive(Clone, PartialEq)]
pub struct ComplexSelectorContent<T: Clone + PartialEq + 'static> {
    /// The current controlled value.
    pub value: T,
    /// Whether the popup is open.
    pub is_open: bool,
    /// Request a value change without dismissing the popup.
    pub on_change: EventHandler<T>,
    /// Dismiss the popup and restore focus from its content.
    pub close: EventHandler<()>,
}

/// A labeled selector for a caller-defined value and popup layout.
#[component]
pub fn ComplexSelector<T: Clone + PartialEq + 'static>(
    label: String,
    value: T,
    content: Callback<ComplexSelectorContent<T>, Element>,
    on_change: EventHandler<T>,
    #[props(default)] trigger_label: Option<Element>,
    #[props(default = "Select an option".to_owned())] placeholder: String,
    #[props(default)] description: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_disabled: bool,
    #[props(default = LayerPlacement::Below)] placement: LayerPlacement,
    #[props(default)] on_open_change: Option<EventHandler<bool>>,
) -> Element {
    let id = use_hook(|| format!("astryx-complex-selector-{}", current_scope_id().0));
    let (is_open, request, onmounted) = use_popover(id.clone(), None, on_open_change);
    let trigger_id = format!("{id}-trigger");
    let description_id = description
        .as_ref()
        .filter(|text| !text.is_empty())
        .map(|_| format!("{trigger_id}-desc"));
    let change = use_callback(move |next: T| {
        if !is_disabled {
            on_change.call(next);
        }
    });
    let close = use_callback(move |()| request.call(false));
    use_effect(use_reactive((&is_disabled,), move |(disabled,)| {
        if disabled {
            request.call(false);
        }
    }));
    let panel = content.call(ComplexSelectorContent {
        value,
        is_open: is_open && !is_disabled,
        on_change: change,
        close,
    });
    let trigger_content = trigger_label.unwrap_or_else(|| rsx! { "{placeholder}" });
    rsx! {
        Field { label: label.clone(), input_id: trigger_id.clone(), description, description_id: description_id.clone(),
            width, is_label_hidden, is_disabled,
            button { id: trigger_id.clone(), class: "astryx-complex-selector__trigger", r#type: "button", role: "combobox",
                "aria-label": label.clone(), "aria-describedby": description_id, "aria-haspopup": "dialog",
                "aria-expanded": (is_open && !is_disabled).to_string(), "aria-controls": id.clone(),
                "aria-disabled": is_disabled.then_some("true"),
                onclick: move |_| { if !is_disabled { request.call(!is_open); } },
                onkeydown: move |event: KeyboardEvent| {
                    if !is_disabled && !event.is_composing() && event.key() == Key::ArrowDown {
                        event.prevent_default();
                        request.call(true);
                    }
                },
                {trigger_content} span { "aria-hidden": "true", "⌄" }
            }
            div { id: id.clone(), class: "astryx-popover astryx-complex-selector__popup", popover: "manual", tabindex: -1,
                role: "dialog", "aria-label": label, "data-open": (is_open && !is_disabled).to_string(),
                "data-placement": placement.as_str(), "data-popover-focus": "true", style: "position-anchor:--{id}", onmounted,
                {panel}
            }
        }
    }
}
