//! Input-container delegation for a standalone description pointing to a control by ID.

use dioxus::prelude::*;

use super::use_clickable_container::INTERACTIVE_SELECTORS;

pub(crate) fn use_input_container(
    marker: String,
    input_id: Option<String>,
) -> EventHandler<MountedEvent> {
    let mut controller = use_signal(|| None::<document::Eval>);
    use_effect(use_reactive(&input_id, move |input_id| {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(input_id);
        }
    }));
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(false);
        }
    });
    EventHandler::new(move |_| {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(false);
        }
        let eval = document::eval(include_str!("use_input_container.js"));
        let _ = eval.send((marker.clone(), input_id.clone(), INTERACTIVE_SELECTORS));
        controller.set(Some(eval));
    })
}
