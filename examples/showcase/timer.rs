use dioxus::prelude::*;

use astryx_dioxus::{
    HStack, SpacingStep, Text, TextColor, TextSize, TextType, TextWeight, Timer, TimerFormat,
    VStack,
};

#[component]
pub(super) fn TimerExamples() -> Element {
    let mut baseline = use_signal(|| None::<f64>);
    let mut clock = use_signal(|| false);
    let mut visible = use_signal(|| true);
    use_effect(move || {
        spawn(async move {
            if let Ok(now) = document::eval("return Date.now();").join::<f64>().await {
                baseline.set(Some(now));
            }
        });
    });
    let long = baseline().map(|now| now - 3_753_000.0);
    let short = baseline().map(|now| now - 128_000.0);
    rsx! {
        section { id: "timer-examples", "data-astryx-theme": "neutral",
            h2 { "Timer" }
            h3 { "Showcase" }
            VStack { gap: SpacingStep::Four,
                Text { text_type: TextType::LARGE, weight: TextWeight::Semibold, "Processing" }
                HStack { gap: SpacingStep::Six,
                    for (caption, format) in [("Elapsed", TimerFormat::Elapsed), ("Clock", TimerFormat::Clock)] {
                        VStack { gap: SpacingStep::One,
                            Text { text_type: TextType::SUPPORTING, "{caption}" }
                            Timer { format, start_time: long, text_type: TextType::BODY, color: TextColor::PRIMARY, size: TextSize::ExtraLarge, weight: TextWeight::Semibold }
                        }
                    }
                }
            }
            h3 { "Formats" }
            VStack { gap: SpacingStep::Three,
                HStack { gap: SpacingStep::Three,
                    Text { text_type: TextType::SUPPORTING, "Elapsed" }
                    Timer { start_time: long, text_type: TextType::BODY, color: TextColor::PRIMARY }
                }
                HStack { gap: SpacingStep::Three,
                    Text { text_type: TextType::SUPPORTING, "Clock" }
                    Timer { start_time: long, text_type: TextType::BODY, color: TextColor::PRIMARY, format: TimerFormat::Clock }
                }
            }
            h3 { "Typography" }
            VStack { gap: SpacingStep::Three,
                Text { text_type: TextType::SUPPORTING, "Default" }
                Timer { start_time: short }
                Text { text_type: TextType::SUPPORTING, "Emphasized" }
                Timer { start_time: short, text_type: TextType::BODY, color: TextColor::PRIMARY, size: TextSize::ExtraLarge, weight: TextWeight::Semibold }
            }
            h3 { "Inline" }
            Text { "Processing for " Timer { id: "timer-inline", text_type: TextType::INHERIT, color: TextColor::INHERIT } }
            h3 { "Change format" }
            button { id: "timer-toggle-format", onclick: move |_| clock.toggle(), "Change format" }
            button { id: "timer-toggle-mount", onclick: move |_| visible.toggle(), "Toggle timer" }
            if visible() {
                Timer { id: "timer-reactive", start_time: long, format: if clock() { TimerFormat::Clock } else { TimerFormat::Elapsed } }
            }
        }
    }
}
