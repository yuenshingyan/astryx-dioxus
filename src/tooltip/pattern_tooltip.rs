// Private example integration of the shared Tooltip controller. No Tooltip logic
// is copied here: presentation, interest and dismissal stay with their owners.
use dioxus::prelude::*;

pub(crate) fn use_pattern_tooltip(id: String, name: String) -> EventHandler<MountedEvent> {
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    EventHandler::new(move |_| {
        if let Some(previous) = *controller.peek() {
            let _ = previous.send("dispose");
        }
        let eval = document::eval(concat!(
            include_str!("../private_attributes/layer_root.js"),
            "\n",
            include_str!("../layer/layer_surface_press.js"),
            "\n",
            include_str!("use_tooltip.js"),
            "\n",
            include_str!("../motion/theme_timing.js"),
            "\n",
            include_str!("../hooks/use_press_feedback/press_feedback.js"),
            "\n",
            include_str!("../hooks/use_press_feedback/use_press_feedback.js"),
        ));
        let _ = eval.send((
            id.clone(),
            name.clone(),
            Some(name.clone()),
            false,
            false,
            false,
            false,
        ));
        controller.set(Some(eval));
    })
}
