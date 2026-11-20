use dioxus::prelude::*;

use super::component::InputStatusType;
use crate::hooks::use_announce::ANNOUNCE_CONTROLLER;

pub(super) fn use_field_status_announcement(id: String, message: String, status: InputStatusType) {
    let mut controller = use_signal(|| None::<document::Eval>);
    use_effect(use_reactive!(|(id, message, status)| {
        let eval = if let Some(eval) = *controller.peek() {
            eval
        } else {
            let eval = document::eval(&format!(
                "{}\n{}",
                ANNOUNCE_CONTROLLER,
                include_str!("use_field_status_announcement.js")
            ));
            let _ = eval.send(id);
            controller.set(Some(eval));
            eval
        };
        let _ = eval.send((message, status == InputStatusType::Error));
    }));
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(false);
        }
    });
}
