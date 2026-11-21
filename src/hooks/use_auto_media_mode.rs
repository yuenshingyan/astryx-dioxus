use dioxus::prelude::*;

pub(crate) fn use_auto_media_mode(
    id: String,
    mode: &'static str,
    fallback: &'static str,
) -> EventHandler<MountedEvent> {
    let mut controller = use_signal(|| None::<document::Eval>);
    let mut is_mounted = use_signal(|| false);
    use_effect(use_reactive(&(mode, fallback), {
        let id = id.clone();
        move |(mode, fallback)| {
            if !is_mounted() {
                return;
            }
            if mode != "auto" {
                if let Some(eval) = *controller.peek() {
                    let _ = eval.send(false);
                }
                controller.set(None);
                return;
            }
            let previous = *controller.peek();
            if let Some(eval) = previous {
                let _ = eval.send(fallback);
            } else {
                let eval = document::eval(include_str!("use_auto_media_mode.js"));
                let _ = eval.send((id.clone(), fallback));
                controller.set(Some(eval));
            }
        }
    }));
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(false);
        }
    });
    EventHandler::new(move |_| is_mounted.set(true))
}
