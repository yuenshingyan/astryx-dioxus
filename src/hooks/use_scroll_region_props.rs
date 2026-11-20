use dioxus::prelude::*;

pub(crate) fn use_scroll_region(
    marker: &'static str,
    id: String,
    is_span: bool,
    should_measure: bool,
) -> bool {
    let mut overflows = use_signal(|| false);
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let mut generation = use_signal(|| 0_u64);
    use_effect(use_reactive(
        (&id, &is_span, &should_measure),
        move |(id, _, should_measure)| {
            let current = generation.peek().wrapping_add(1);
            generation.set(current);
            if let Some(eval) = controller.take() {
                let _ = eval.send(());
            }
            overflows.set(false);
            if !should_measure {
                return;
            }
            let mut eval = document::eval(concat!(
                include_str!("../utils/shared_resize_observer.js"),
                "\n",
                include_str!("use_scroll_region_props.js"),
            ));
            let _ = eval.send((marker, id));
            controller.set(Some(eval));
            spawn(async move {
                while let Ok(value) = eval.recv::<bool>().await {
                    if *generation.peek() != current {
                        break;
                    }
                    overflows.set(value);
                }
            });
        },
    ));
    should_measure && overflows()
}
