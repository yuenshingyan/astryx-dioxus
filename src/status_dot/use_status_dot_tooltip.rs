// StatusDot-owned native placement/activation adapter. Tooltip owns its surface.

use dioxus::prelude::*;

use crate::hooks::use_announce::ANNOUNCE_CONTROLLER;
use crate::tooltip::use_tooltip::TOOLTIP_CONTROLLER;
use crate::utils::hit_area::HIT_AREA_CONTROLLER;
use crate::utils::touch_hit_area::TOUCH_HIT_AREA_CONTROLLER;

/// Renderer boundary for the internal StatusDot trigger. Compose caller handlers
/// first, then these handlers, so cancellation wins and a pin never activates a
/// Dioxus ancestor. StatusDot owns ancestor placement semantics; shared Tooltip
/// owns surface visibility, while each native owner cleans up its own resources.
pub(super) struct StatusDotTooltipSupport {
    pub onmounted: EventHandler<MountedEvent>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onkeyup: EventHandler<KeyboardEvent>,
}

type StatusDotConfig = (String, String, Option<String>, Vec<String>);

fn start_status_dot_controller(
    config: &StatusDotConfig,
    mut is_standalone: Signal<bool>,
) -> document::Eval {
    let mut controller = document::eval(&format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        ANNOUNCE_CONTROLLER,
        HIT_AREA_CONTROLLER,
        TOUCH_HIT_AREA_CONTROLLER,
        include_str!("status_dot.js"),
        TOOLTIP_CONTROLLER,
        include_str!("../hooks/use_press_feedback/press_feedback.js"),
        include_str!("use_status_dot_tooltip.js"),
    ));
    let _ = controller.send(config.clone());
    spawn(async move {
        while let Ok(placement) = controller.recv::<bool>().await {
            is_standalone.set(placement);
        }
    });
    controller
}

/// Internal StatusDot FR5/AR2 adapter, not a public Tooltip component.
///
/// Call unconditionally. Only nonempty tooltip strings attach native machinery.
/// Put `data-astryx-status-tooltip-id=id` on that root, initially role=img with
/// label and no generated tabstop/ARIA. The shared manager lazily owns the
/// surface; do not place a prerendered flow-content surface inside a control.
/// `caller_attribute_names` protects explicit caller props from generated values.
/// Native mount/reparenting selects standalone button vs nested hover-only image.
pub(super) fn use_status_dot_tooltip(
    id: String,
    label: String,
    tooltip: Option<String>,
    caller_attribute_names: Vec<String>,
) -> StatusDotTooltipSupport {
    let mut controller = use_signal(|| None::<document::Eval>);
    let mut is_mounted = use_signal(|| false);
    let mut is_standalone = use_signal(|| false);
    let tooltip = tooltip.filter(|text| !text.trim().is_empty());
    let should_attach = tooltip.is_some();
    let config = (id, label, tooltip, caller_attribute_names);
    use_drop(move || {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    use_effect(use_reactive(&config, move |config| {
        if !is_mounted() {
            return;
        }
        let previous = *controller.peek();
        if config.2.is_none() {
            if let Some(previous) = previous {
                let _ = previous.send("dispose");
                controller.set(None);
            }
            is_standalone.set(false);
        } else if let Some(previous) = previous {
            let _ = previous.send(config);
        } else {
            controller.set(Some(start_status_dot_controller(&config, is_standalone)));
        }
    }));
    StatusDotTooltipSupport {
        onmounted: EventHandler::new(move |_| {
            if let Some(previous) = *controller.peek() {
                let _ = previous.send("dispose");
            }
            is_standalone.set(false);
            controller
                .set(should_attach.then(|| start_status_dot_controller(&config, is_standalone)));
            is_mounted.set(true);
        }),
        onclick: EventHandler::new(move |event: MouseEvent| {
            if !should_attach || !*is_standalone.peek() {
                return;
            }
            event.stop_propagation();
            if !event.default_action_enabled() {
                return;
            }
            event.prevent_default();
            if let Some(controller) = *controller.peek() {
                let _ = controller.send("toggle");
            }
        }),
        onkeydown: EventHandler::new(move |event: KeyboardEvent| {
            if !should_attach || !*is_standalone.peek() || event.is_composing() {
                return;
            }
            let command = match event.key() {
                Key::Enter => "enter",
                Key::Character(key) if key == " " => "space-down",
                _ => return,
            };
            event.stop_propagation();
            if !event.default_action_enabled() {
                return;
            }
            event.prevent_default();
            if !event.is_auto_repeating()
                && let Some(controller) = *controller.peek()
            {
                let _ = controller.send(command);
            }
        }),
        onkeyup: EventHandler::new(move |event: KeyboardEvent| {
            if !should_attach || !*is_standalone.peek() || event.is_composing() {
                return;
            }
            if event.key() == Key::Enter {
                event.stop_propagation();
                if event.default_action_enabled() {
                    event.prevent_default();
                }
                return;
            }
            if event.key() != Key::Character(" ".into()) {
                return;
            }
            event.stop_propagation();
            let command = if event.default_action_enabled() {
                event.prevent_default();
                "space-up"
            } else {
                "space-cancel"
            };
            if let Some(controller) = *controller.peek() {
                let _ = controller.send(command);
            }
        }),
    }
}
