// ProgressBarTouch-owned native adapter for the current ProgressBar contract.
// Based on Meta Astryx 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md. Measured UIKit timing is reference-only.

use dioxus::prelude::*;

/// Internal finite mode passed across the native motion boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ProgressBarMotionMode {
    Determinate,
    Indeterminate,
}

impl ProgressBarMotionMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Determinate => "determinate",
            Self::Indeterminate => "indeterminate",
        }
    }
}

type MotionConfig = (String, f64, &'static str, bool);

fn start_controller(config: &MotionConfig) -> document::Eval {
    let controller = document::eval(&format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        include_str!("../motion/uikit/progress_bar.js"),
        include_str!("../motion/uikit/timing.js"),
        include_str!("../motion/theme_timing.js"),
        include_str!("../utils/touch_queries.js"),
        include_str!("../utils/shared_resize_observer.js"),
        include_str!("progress_bar_touch.js"),
    ));
    let _ = controller.send(config.clone());
    controller
}

/// Call unconditionally; receives a percentage in 0..100, not a 0..1 fraction.
/// Compose this on the stable data-astryx-progress-bar-id root. The caller's
/// mounted callback runs after native attachment is requested. SSR renders its
/// target directly; attachment never introduces a mount animation.
///
/// The native channel owns only private motion fields and mark placement. CSS
/// owns geometry, paint, direction mirroring and indeterminate animation. Drop
/// closes native resources; mode changes cancel determinate motion and resize
/// observation. No per-frame Rust task or DOM writes are created.
pub(super) fn use_progress_bar_touch(
    id: String,
    percentage: f64,
    mode: ProgressBarMotionMode,
    has_marks: bool,
    onmounted: Option<EventHandler<MountedEvent>>,
) -> EventHandler<MountedEvent> {
    let percentage = if percentage.is_finite() {
        percentage.clamp(0.0, 100.0)
    } else {
        0.0
    };
    let config = (id, percentage, mode.as_str(), has_marks);
    let mut controller = use_signal(|| None::<(document::Eval, &'static str)>);
    let mut is_mounted = use_signal(|| false);
    use_drop(move || {
        if let Some((previous, _)) = *controller.peek() {
            let _ = previous.send("dispose");
        }
    });
    use_effect(use_reactive(&config, move |config| {
        if !is_mounted() {
            return;
        }
        let previous = *controller.peek();
        if let Some((previous, previous_mode)) = previous {
            if previous_mode == config.2 {
                let _ = previous.send(config);
            } else {
                // A mode owns its channel and all native listeners/observers.
                // Start the new mode only after requesting the old teardown.
                let _ = previous.send("dispose");
                controller.set(Some((start_controller(&config), config.2)));
            }
        }
    }));
    EventHandler::new(move |event| {
        if let Some((previous, _)) = *controller.peek() {
            let _ = previous.send("dispose");
        }
        controller.set(Some((start_controller(&config), config.2)));
        is_mounted.set(true);
        if let Some(caller) = onmounted {
            caller.call(event);
        }
    })
}
