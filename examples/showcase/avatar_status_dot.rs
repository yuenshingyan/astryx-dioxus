use dioxus::prelude::*;

use astryx_dioxus::{
    Avatar, AvatarSize, AvatarStatusDot, AvatarStatusDotVariant, HStack, SpacingStep,
    StackCrossAlignment,
};

#[component]
pub(super) fn AvatarStatusDotExamples() -> Element {
    rsx! {
        section { id: "avatar-status-dot-examples", "data-astryx-theme": "neutral",
            h2 { "AvatarStatusDot" }
            h3 { "Showcase" }
            HStack { gap: SpacingStep::Three,
                Avatar { name: "Online User", size: AvatarSize::ExtraLarge, status: rsx! { AvatarStatusDot { label: "Online", variant: AvatarStatusDotVariant::SUCCESS } } }
                Avatar { name: "Away User", size: AvatarSize::ExtraLarge, status: rsx! { AvatarStatusDot { label: "Away", variant: AvatarStatusDotVariant::NEUTRAL } } }
                Avatar { name: "Busy User", size: AvatarSize::ExtraLarge, status: rsx! { AvatarStatusDot { label: "Busy", variant: AvatarStatusDotVariant::ERROR } } }
            }
            h3 { "Variants" }
            HStack { gap: SpacingStep::Three,
                Avatar { name: "Ana Silva", size: AvatarSize::ExtraLarge, status: rsx! { AvatarStatusDot { label: "Online", variant: AvatarStatusDotVariant::SUCCESS } } }
                Avatar { name: "Ben Okafor", size: AvatarSize::ExtraLarge, status: rsx! { AvatarStatusDot { label: "Away", variant: AvatarStatusDotVariant::NEUTRAL } } }
                Avatar { name: "Cleo Marsh", size: AvatarSize::ExtraLarge, status: rsx! { AvatarStatusDot { label: "Do not disturb", variant: AvatarStatusDotVariant::ERROR } } }
            }
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
