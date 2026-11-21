//! Adapted from Astryx Timer/Timer.tsx at 476a3153417dbf75c565f7df05b0784e94ddd59e.

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::text::Text;
use crate::theme::{TextColor, TextSize, TextType, TextWeight};

/// Standard elapsed-duration display.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimerFormat {
    /// Compact seconds, minutes and seconds, or hours and minutes.
    #[default]
    Elapsed,
    /// Stopwatch-style minutes and seconds, with hours when needed.
    Clock,
}

/// Displays an elapsed duration without rerendering for each tick.
/// The renderer-neutral initial value is zero; the browser starts the clock on mount.
#[component]
pub fn Timer(
    /// Start time in Unix milliseconds, or the mount time when omitted or nonfinite.
    #[props(default)]
    start_time: Option<f64>,
    /// Display format and precision.
    #[props(default)]
    format: TimerFormat,
    /// Semantic typography applied to the outer Text.
    #[props(default = TextType::SUPPORTING)]
    text_type: TextType,
    /// Explicit font size.
    #[props(default)]
    size: Option<TextSize>,
    /// Text color applied to the outer Text.
    #[props(default = TextColor::SECONDARY)]
    color: TextColor,
    /// Font weight override.
    #[props(default)]
    weight: Option<TextWeight>,
    /// Caller classes on the outer Text.
    #[props(default)]
    class: String,
    /// Caller inline CSS on the outer Text.
    #[props(default)]
    style: String,
    /// Mounted callback for the native time element.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native attributes on the time element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let id = format!("timer-{}", current_scope_id().0);
    let config = (
        start_time.filter(|value| value.is_finite()),
        format == TimerFormat::Clock,
    );
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(false);
        }
    });
    use_effect(use_reactive(&config, move |config| {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(config);
        }
    }));
    let mount_id = id.clone();
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class" | "style" | "datetime" | "data-astryx-timer-id" | "onmounted"
            )
        })
        .collect();
    // The fixed zero text node is owned by the timer bridge after mount. Even
    // a format change leaves its identity intact for subsequent writes.
    let initial = if format == TimerFormat::Clock {
        "0:00"
    } else {
        "0s"
    };
    let initial = use_hook(move || initial);
    rsx! {
        Text { text_type, size, color, weight, class: "astryx-timer {class}", style,
            time { class: "astryx-timer__time", "datetime": "PT0S", "data-astryx-timer-id": id,
                onmounted: move |event| {
                    if let Some(previous) = *controller.peek() { let _ = previous.send(false); }
                    let eval = document::eval(include_str!("timer.js"));
                    let _ = eval.send((mount_id.clone(), config));
                    controller.set(Some(eval));
                    if let Some(callback) = onmounted { callback.call(event); }
                },
                ..attributes,
                "{initial}"
            }
        }
    }
}
