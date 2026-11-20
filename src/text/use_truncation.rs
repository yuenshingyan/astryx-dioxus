// Adapted from Meta Astryx Text/useTruncation.ts at
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

/// Browser overflow measurement, independent of tooltip presentation.
///
/// Attach `onmounted` to the root carrying `data-astryx-truncation-id` equal
/// to the hook's `id`. Compose it with any caller mounted handler. Measurement
/// waits for the shared ResizeObserver's first layout notification, then follows
/// resize and content changes. Zero lines installs no observers.
#[derive(Clone, Copy)]
pub struct Truncation {
    /// Native mount adapter, replacing the upstream callback ref.
    pub onmounted: EventHandler<MountedEvent>,
    /// Whether the last clamped measurement overflowed.
    pub is_truncated: ReadSignal<bool>,
    /// Full `textContent` from the last measurement, including rich descendants.
    pub full_text: ReadSignal<String>,
}

type Config = (String, u32, bool);

fn start_controller(
    config: &Config,
    mut is_truncated: Signal<bool>,
    mut full_text: Signal<String>,
    mut is_plain_overflow: Signal<bool>,
) -> document::Eval {
    let mut controller = document::eval(concat!(
        include_str!("../utils/shared_resize_observer.js"),
        "\n",
        include_str!("../utils/shared_mutation_observer.js"),
        "\n",
        include_str!("use_truncation.js"),
        "\n",
        include_str!("use_truncation_channel.js"),
    ));
    let _ = controller.send(config.clone());
    spawn(async move {
        while let Ok((is_cut, text, is_plain_cut)) = controller.recv::<(bool, String, bool)>().await
        {
            if *is_plain_overflow.peek() != is_plain_cut {
                is_plain_overflow.set(is_plain_cut);
            }
            if *is_truncated.peek() != is_cut {
                is_truncated.set(is_cut);
            }
            if *full_text.peek() != text {
                full_text.set(text);
            }
        }
    });
    controller
}

/// Detects overflow for Text, Heading and caller-owned advanced text surfaces.
///
/// `id` must identify one mounted root through `data-astryx-truncation-id`.
/// With `is_expanded`, retain the last clamped answer without measuring the
/// expanded content. Replacing the expanded root discards that answer when
/// clamped again. This hook never changes styles, adds a tab stop, or creates
/// a tooltip. The browser channel and shared subscriptions are released on drop.
pub fn use_truncation(id: String, max_lines: u32, is_expanded: bool) -> Truncation {
    use_truncation_measurement(id, max_lines, is_expanded).0
}

pub(super) fn use_truncation_measurement(
    id: String,
    max_lines: u32,
    is_expanded: bool,
) -> (Truncation, ReadSignal<bool>) {
    let mut controller = use_signal(|| None::<document::Eval>);
    let mut is_mounted = use_signal(|| false);
    let mut is_truncated = use_signal(|| false);
    let mut full_text = use_signal(String::new);
    let mut is_plain_overflow = use_signal(|| false);
    let config = (id, max_lines, is_expanded);
    use_drop(move || {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    use_effect(use_reactive(&config, move |config| {
        if !is_mounted() {
            return;
        }
        if let Some(previous) = *controller.peek() {
            let _ = previous.send(config);
        } else if config.1 > 0 {
            controller.set(Some(start_controller(
                &config,
                is_truncated,
                full_text,
                is_plain_overflow,
            )));
        } else {
            is_truncated.set(false);
            full_text.set(String::new());
            is_plain_overflow.set(false);
        }
    }));
    (
        Truncation {
            onmounted: EventHandler::new(move |_| {
                if let Some(previous) = *controller.peek() {
                    let _ = previous.send(config.clone());
                } else if max_lines > 0 {
                    controller.set(Some(start_controller(
                        &config,
                        is_truncated,
                        full_text,
                        is_plain_overflow,
                    )));
                }
                is_mounted.set(true);
            }),
            is_truncated: is_truncated.into(),
            full_text: full_text.into(),
        },
        is_plain_overflow.into(),
    )
}
