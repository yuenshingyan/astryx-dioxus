use dioxus::prelude::*;

pub(super) fn use_chat_stream_scroll(
    id: String,
) -> (bool, EventHandler<MountedEvent>, EventHandler<()>) {
    let mut is_scrolled_up = use_signal(|| false);
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(false);
        }
    });
    let mounted = EventHandler::new(move |_: MountedEvent| {
        if controller.peek().is_some() {
            return;
        }
        let mut eval = document::eval(concat!(
            include_str!("../utils/shared_resize_observer.js"),
            "\n",
            include_str!("use_chat_stream_scroll.js"),
        ));
        let _ = eval.send(id.clone());
        controller.set(Some(eval));
        spawn(async move {
            while let Ok(value) = eval.recv::<bool>().await {
                is_scrolled_up.set(value);
            }
        });
    });
    let scroll_to_end = EventHandler::new(move |()| {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(true);
        }
    });
    (is_scrolled_up(), mounted, scroll_to_end)
}
