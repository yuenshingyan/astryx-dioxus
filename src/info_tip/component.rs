use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::hooks::use_announce::ANNOUNCE_CONTROLLER;
use crate::hooks::use_press_feedback::PRESS_FEEDBACK_CONTROLLER;
use crate::tooltip::use_tooltip::{TOOLTIP_CONTROLLER, tooltip_element};
use crate::{Icon, IconSize};

/// Icon size for the help trigger.
pub type InfoTipSize = IconSize;

/// Reveals noninteractive help on hover or focus, and pins it on activation.
#[component]
pub fn InfoTip(
    /// Short, noninteractive help text.
    content: String,
    /// Accessible name of the trigger.
    #[props(default = "More information".to_owned())]
    label: String,
    #[props(default = IconSize::Small)] size: InfoTipSize,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| format!("astryx-info-tip-{}", current_scope_id().0));
    let mut controller = use_signal(|| None::<document::Eval>);
    use_effect(use_reactive(&content, move |content| {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send([content]);
        }
    }));
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send("dispose");
        }
    });
    let mount_id = id.clone();
    let mount_content = content.clone();
    let attributes = attributes
        .into_iter()
        .filter(|attr| attr.name.starts_with("data-") && attr.name != "data-astryx-info-tip-id")
        .collect::<Vec<_>>();
    rsx! {
        button {
            r#type: "button", class: "astryx-info-tip {class}", style,
            "data-astryx-info-tip-id": id.clone(), "aria-label": label,
            "aria-describedby": (!content.trim().is_empty()).then(|| format!("{id}-tooltip")),
            "aria-expanded": "false",
            onmounted: move |event| {
                if let Some(previous) = *controller.peek() { let _ = previous.send("dispose"); }
                let eval = document::eval(&format!(
                    "{ANNOUNCE_CONTROLLER}\n{TOOLTIP_CONTROLLER}\n{PRESS_FEEDBACK_CONTROLLER}\n{}",
                    include_str!("info_tip.js"),
                ));
                let _ = eval.send((mount_id.clone(), mount_content.clone()));
                controller.set(Some(eval));
                if let Some(handler) = onmounted { handler.call(event); }
            },
            onclick: move |event| {
                event.stop_propagation();
                event.prevent_default();
                if let Some(eval) = *controller.peek() { let _ = eval.send("toggle"); }
            },
            ..attributes,
            Icon { size, icon: rsx! {
                svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round",
                    circle { cx: "12", cy: "12", r: "9" }
                    path { d: "M12 11v5m0-8h.01" }
                }
            } }
        }
        {tooltip_element(&id, (!content.trim().is_empty()).then_some(content.as_str()))}
    }
}
