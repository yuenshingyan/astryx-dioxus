use dioxus::prelude::*;

pub(crate) fn use_popover(
    id: String,
    is_open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
) -> (bool, EventHandler<bool>, EventHandler<MountedEvent>) {
    let mut local = use_signal(|| false);
    let open = is_open.unwrap_or_else(|| *local.read());
    let request = use_callback(move |next| {
        if next != open {
            if is_open.is_none() {
                local.set(next);
            }
            if let Some(handler) = on_open_change {
                handler.call(next);
            }
        }
    });
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let mount = use_callback(move |_event: MountedEvent| {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
        let mut eval = document::eval(include_str!("use_popover.js"));
        let _ = eval.send(id.clone());
        controller.set(Some(eval));
        spawn(async move {
            while eval.recv::<bool>().await.is_ok() {
                request.call(false);
            }
        });
    });
    (open, request, mount)
}
