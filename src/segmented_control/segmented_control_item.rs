use dioxus::prelude::*;

use super::segmented_control_context::SegmentedControlContext;

/// One radio option inside a [`SegmentedControl`](super::component::SegmentedControl).
#[component]
pub fn SegmentedControlItem(
    value: String,
    label: String,
    #[props(default)] icon: Option<Element>,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let Some(ctx) = try_consume_context::<SegmentedControlContext>() else {
        return VNode::empty();
    };
    let settings = (ctx.0)();
    let is_selected = settings.value == value;
    let is_disabled = is_disabled || settings.is_disabled;
    rsx! {
        button { class: "astryx-segmented-control__item", r#type: "button", role: "radio",
            "aria-checked": is_selected.to_string(), "aria-disabled": is_disabled.then_some("true"),
            "aria-label": is_label_hidden.then(|| label.clone()), "data-value": value.clone(),
            tabindex: if is_selected { 0 } else { -1 },
            onclick: move |event: MouseEvent| {
                if is_disabled { event.prevent_default(); return; }
                if let Some(handler) = onclick { handler.call(event.clone()); }
                if !is_selected && event.default_action_enabled() { settings.on_change.call(value.clone()); }
            },
            if let Some(icon) = icon { span { class: "astryx-segmented-control__icon", "aria-hidden": "true", {icon} } }
            if !is_label_hidden { span { class: "astryx-segmented-control__label", "{label}" } }
        }
    }
}
