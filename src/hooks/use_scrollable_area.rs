use dioxus::prelude::*;

pub(crate) fn use_scrollable_area(
    id: String,
    config: (&'static str, bool, bool),
) -> EventHandler<MountedEvent> {
    let mut controller = use_signal(|| None::<document::Eval>);
    use_effect(use_reactive(&config, move |config| {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(config);
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
        let eval = document::eval(include_str!("use_scrollable_area.js"));
        let _ = eval.send((id.clone(), config));
        controller.set(Some(eval));
    })
}
