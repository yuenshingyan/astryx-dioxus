use dioxus::prelude::*;

use astryx_dioxus::{
    AvatarStatusDot, AvatarStatusDotVariant, HStack, SpacingStep, StackCrossAlignment,
};

#[component]
pub(super) fn AvatarStatusDotExamples() -> Element {
    rsx! {
        section { id: "avatar-status-dot-examples", "data-astryx-theme": "neutral",
            h2 { "AvatarStatusDot" }
            p { "Standalone status dots use the 36px avatar size context by default." }
            HStack { gap: SpacingStep::Three, v_align: StackCrossAlignment::Center,
                for (id, label, variant) in [
                    ("avatar-dot-online", "Online", AvatarStatusDotVariant::SUCCESS),
                    ("avatar-dot-away", "Away", AvatarStatusDotVariant::NEUTRAL),
                    ("avatar-dot-busy", "Do not disturb", AvatarStatusDotVariant::ERROR),
                ] {
                    HStack { key: "{id}", gap: SpacingStep::One, v_align: StackCrossAlignment::Center,
                        AvatarStatusDot { id, label, variant }
                        span { "{label}" }
                    }
                }
            }
        }
    }
}
