use dioxus::prelude::*;

use astryx_dioxus::{Avatar, AvatarGroup, AvatarGroupOverflow, AvatarSize};

const MEMBERS: [(&str, &str); 3] = [
    ("Alex Daniels", "/team/alex"),
    ("Ann Smith", "/team/ann"),
    ("Carol Davis", "/team/carol"),
];

#[component]
pub(super) fn AvatarGroupExamples() -> Element {
    let mut activated = use_signal(|| false);
    let mut block_arrows = use_signal(|| false);
    rsx! {
        section { id: "avatar-group-examples", "data-astryx-theme": "neutral",
            h2 { "AvatarGroup" }
            h3 { "Showcase" }
            AvatarGroup { size: AvatarSize::Large, label: "All members",
                for name in ["Alex Daniels", "Ann Smith", "Carol Davis", "Gina Wilson", "Eve Park"] {
                    Avatar { key: "{name}", name }
                }
            }
            AvatarGroup { size: AvatarSize::Large, label: "Selected members",
                for (name, _) in MEMBERS { Avatar { key: "{name}", name } }
                AvatarGroupOverflow { count: 2 }
            }
            h3 { "Interactive" }
            button { id: "avatar-group-toggle-keys", onclick: move |_| block_arrows.toggle(), "Toggle arrow handling" }
            p { id: "avatar-group-description", "Reviewers" }
            AvatarGroup { id: "avatar-group-reviewers", size: AvatarSize::Large, label: "Reviewers",
                "aria-describedby": "avatar-group-description",
                onkeydown: move |event: KeyboardEvent| {
                    if block_arrows() && event.key() == Key::ArrowRight { event.prevent_default(); }
                },
                for (name, href) in MEMBERS { Avatar { key: "{name}", name, href } }
                AvatarGroupOverflow { count: 15, on_click: move |_| activated.set(true) }
            }
            if activated() { output { id: "avatar-group-result", "More reviewers" } }
            p { "Compact, static" }
            AvatarGroup { id: "avatar-group-attendees", size: AvatarSize::Small, label: "Attendees",
                for (name, _) in MEMBERS { Avatar { key: "{name}", name } }
                AvatarGroupOverflow { count: 15 }
            }
        }
    }
}
