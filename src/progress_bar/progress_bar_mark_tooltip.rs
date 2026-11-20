// Adapted from ProgressBar/ProgressBarMarkTooltip.tsx. Tooltip registration is
// deferred until a mark exists. This is not a public Tooltip implementation.
use dioxus::prelude::*;

use crate::tooltip::use_tooltip::TOOLTIP_CONTROLLER;

fn start_controller(config: &(String, String)) -> document::Eval {
    let controller = document::eval(&format!(
        "{}\n{}\n{}",
        TOOLTIP_CONTROLLER,
        include_str!("../hooks/use_press_feedback/press_feedback.js"),
        include_str!("progress_bar_mark_tooltip.js")
    ));
    let _ = controller.send(config.clone());
    controller
}

pub(super) fn use_progress_bar_mark_tooltip(
    id: String,
    label: String,
) -> EventHandler<MountedEvent> {
    let mut controller = use_signal(|| None::<document::Eval>);
    let config = (id, label);
    use_drop(move || {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    use_effect(use_reactive(&config, move |config| {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send(config);
        }
    }));
    EventHandler::new(move |_| {
        if let Some(previous) = *controller.peek() {
            let _ = previous.send("dispose");
        }
        controller.set(Some(start_controller(&config)));
    })
}
