use dioxus::prelude::*;

use crate::{Button, ButtonSize, ButtonVariant, Icon, IconSize};

/// Listening state and meter samples supplied by the dictation owner.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ChatDictationState {
    #[default]
    Unsupported,
    Idle,
    Listening {
        bands: [f32; 5],
        volume: f32,
    },
}

/// Supported sizes for the microphone control and its meter.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChatDictationButtonSize {
    Small,
    #[default]
    Medium,
}

/// Microphone toggle. The caller owns recording, permissions, and samples.
#[component]
pub fn ChatDictationButton(
    dictation: ChatDictationState,
    on_toggle: EventHandler<()>,
    #[props(default)] size: ChatDictationButtonSize,
    #[props(default = true)] is_hidden_when_unsupported: bool,
    #[props(default)] label: Option<String>,
    #[props(default = "Start dictation".to_owned())] start_label: String,
    #[props(default = "Stop dictation".to_owned())] stop_label: String,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let is_unsupported = dictation == ChatDictationState::Unsupported;
    if is_unsupported && is_hidden_when_unsupported {
        return rsx! {};
    }
    let is_listening = matches!(dictation, ChatDictationState::Listening { .. });
    let label = label.unwrap_or(if is_listening {
        stop_label
    } else {
        start_label
    });
    let (button_size, icon_size, size_name) = match size {
        ChatDictationButtonSize::Small => (ButtonSize::Small, IconSize::Small, "sm"),
        ChatDictationButtonSize::Medium => (ButtonSize::Medium, IconSize::Medium, "md"),
    };
    let attributes = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "data-size"))
        .collect::<Vec<_>>();
    rsx! {
        span { class: "astryx-chat-dictation-button {class}", style, "data-size": size_name, ..attributes,
            if let ChatDictationState::Listening { bands, volume } = dictation {
                DictationMeter { bands, volume }
            }
            Button { label, size: button_size, variant: ButtonVariant::Ghost, is_icon_only: true, is_disabled: is_unsupported,
                onclick: move |_| on_toggle.call(()),
                icon: (!is_listening).then(|| rsx! { Icon { size: icon_size, icon: rsx! {
                    svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
                        rect { x: "9", y: "3", width: "6", height: "12", rx: "3" }
                        path { d: "M5 11a7 7 0 0 0 14 0m-7 7v3m-3 0h6" }
                    }
                } } }),
            }
        }
    }
}

#[component]
fn DictationMeter(bands: [f32; 5], volume: f32) -> Element {
    let clipping = ((sample(volume) - 0.2) / 0.1).clamp(0.0, 1.0) * 100.0;
    rsx! {
        span { class: "astryx-chat-dictation-button__meter", "aria-hidden": "true", style: "--_dictation-clip:{clipping}%;",
            for (index, level) in bands.into_iter().enumerate() {
                span { key: "{index}", class: "astryx-chat-dictation-button__bar", style: "--_dictation-scale:{scale(level)};" }
            }
        }
    }
}

fn sample(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn scale(value: f32) -> f32 {
    0.08 + (sample(value) / 0.2).sqrt().min(1.0) * 0.92
}
