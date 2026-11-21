use dioxus::prelude::*;

use astryx_dioxus::{Avatar, AvatarGroup, AvatarGroupOverflow, AvatarSize};

#[component]
fn OverflowRow(count: i64, #[props(default)] custom_text: bool) -> Element {
    rsx! {
        AvatarGroup { size: AvatarSize::Large,
            for name in ["Alex Daniels", "Ann Smith", "Carol Davis"] {
                Avatar { key: "{name}", name }
            }
            AvatarGroupOverflow { count, children: custom_text.then(|| rsx! { "12+" }) }
        }
    }
}

#[component]
pub(super) fn AvatarGroupOverflowExamples() -> Element {
    let mut activations = use_signal(|| 0);
    rsx! {
        section { id: "avatar-group-overflow-examples", "data-astryx-theme": "neutral",
            h2 { "AvatarGroupOverflow" }
            h3 { "Showcase" }
            OverflowRow { count: 5 }
            OverflowRow { count: 12, custom_text: true }
            h3 { "Default" }
            OverflowRow { count: 2 }
            h3 { "Custom text" }
            OverflowRow { count: 12, custom_text: true }
            h3 { "Standalone and interactive" }
            AvatarGroupOverflow { id: "avatar-overflow-zero", count: -3 }
            AvatarGroupOverflow { id: "avatar-overflow-large", count: 4912 }
            AvatarGroupOverflow { id: "avatar-overflow-action", count: 3, on_click: move |_| activations += 1 }
            output { id: "avatar-overflow-activations", "{activations}" }
        }
    }
}
