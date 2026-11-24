use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::segmented_control_context::{SegmentedControlContext, Settings};
use crate::{ElementSize, use_size};

/// Inline sizing of the segment track.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SegmentedControlLayout {
    /// Fit the segments to their content.
    #[default]
    Hug,
    /// Divide the available width among the segments.
    Fill,
}

/// A named radio group whose selected value is owned by the caller.
#[component]
pub fn SegmentedControl(
    label: String,
    value: String,
    on_change: EventHandler<String>,
    children: Element,
    #[props(default)] layout: SegmentedControlLayout,
    #[props(default)] size: Option<ElementSize>,
    #[props(default)] is_disabled: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-segmented-control-{}", current_scope_id().0));
    let mount_id = id.clone();
    let size = use_size(size);
    let settings = Settings {
        value,
        on_change,
        is_disabled,
    };
    let SegmentedControlContext(mut context) =
        use_context_provider(|| SegmentedControlContext(Signal::new(settings.clone())));
    if *context.peek() != settings {
        context.set(settings);
    }
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    rsx! {
        div { id, class: "astryx-segmented-control", role: "radiogroup", "aria-label": label,
            "aria-disabled": is_disabled.then_some("true"), "data-size": size.as_str(),
            "data-fill": (layout == SegmentedControlLayout::Fill).then_some("true"),
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let eval = document::eval(include_str!("segmented_control.js"));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
            },
            {children}
        }
    }
}
