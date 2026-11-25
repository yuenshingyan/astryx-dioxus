use dioxus::prelude::*;

pub(crate) fn use_overflow(id: String) -> (ReadSignal<Option<usize>>, EventHandler<MountedEvent>) {
    let mut count = use_signal(|| None::<usize>);
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let mount = use_callback(move |_: MountedEvent| {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
        let mut eval = document::eval(include_str!("use_overflow.js"));
        let _ = eval.send(id.clone());
        controller.set(Some(eval));
        spawn(async move {
            while let Ok(next) = eval.recv::<usize>().await {
                if *count.peek() != Some(next) {
                    count.set(Some(next));
                }
            }
        });
    });
    (count.into(), mount)
}
