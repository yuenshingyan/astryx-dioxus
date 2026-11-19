// Adapted from Astryx usePressFeedback. See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

use crate::tooltip::use_tooltip::TOOLTIP_CONTROLLER;

/// DOM bridge data; activation and async operations remain caller-owned.
#[derive(Clone, PartialEq)]
pub(crate) struct PressFeedbackConfig {
    pub id: String,
    pub label: String,
    pub tooltip: Option<String>,
    pub is_icon_only: bool,
    pub is_loading: bool,
    pub is_disabled: bool,
}

type WireConfig = (String, String, Option<String>, bool, bool, bool, bool);

fn start_controller(config: &WireConfig) -> document::Eval {
    let controller = document::eval(&format!(
        "{}\n{}\n{}",
        TOOLTIP_CONTROLLER,
        include_str!("press_feedback.js"),
        include_str!("use_press_feedback.js"),
    ));
    let _ = controller.send(config.clone());
    controller
}

/// Mount on a pressable root. Link uses data-astryx-feedback-id; Button retains
/// data-astryx-button-id. This hook neither guards activation nor paints geometry.
pub(crate) fn use_press_feedback(config: PressFeedbackConfig) -> EventHandler<MountedEvent> {
    use_feedback_controller(config, false, true)
}

/// Tooltip-only anchors can remain entirely unregistered while disabled. Changing
/// that option after mount installs/disposes the controller without remounting Text.
pub(crate) fn use_feedback_controller(
    config: PressFeedbackConfig,
    is_truncation: bool,
    should_attach: bool,
) -> EventHandler<MountedEvent> {
    let mut controller = use_signal(|| None::<document::Eval>);
    let mut is_mounted = use_signal(|| false);
    use_drop(move || {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    let config = (
        config.id,
        config.label,
        config.tooltip.filter(|text| !text.trim().is_empty()),
        config.is_icon_only,
        config.is_loading,
        config.is_disabled,
        is_truncation,
    );
    use_effect(use_reactive(
        &(config.clone(), should_attach),
        move |(config, should_attach)| {
            if !is_mounted() {
                return;
            }
            let previous = *controller.peek();
            if !should_attach {
                if let Some(previous) = previous {
                    let _ = previous.send("dispose");
                    controller.set(None);
                }
                return;
            }
            if let Some(previous) = previous {
                let _ = previous.send(config);
            } else {
                controller.set(Some(start_controller(&config)));
            }
        },
    ));
    EventHandler::new(move |_| {
        if let Some(previous) = *controller.peek() {
            let _ = previous.send("dispose");
        }
        controller.set(should_attach.then(|| start_controller(&config)));
        is_mounted.set(true);
    })
}
