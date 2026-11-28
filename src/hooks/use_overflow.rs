use dioxus::prelude::*;

type OverflowMeasurement = (usize, usize);

pub(crate) fn use_overflow(
    id: String,
) -> (
    ReadSignal<Option<OverflowMeasurement>>,
    EventHandler<MountedEvent>,
) {
    let mut measurement = use_signal(|| None::<OverflowMeasurement>);
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
            while let Ok(next) = eval.recv::<OverflowMeasurement>().await {
                if *measurement.peek() != Some(next) {
                    measurement.set(Some(next));
                }
            }
        });
    });
    (measurement.into(), mount)
}
