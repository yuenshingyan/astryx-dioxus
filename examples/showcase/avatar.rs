use dioxus::prelude::*;

use astryx_dioxus::{Avatar, AvatarSize, AvatarStatusDot, AvatarStatusDotVariant, AvatarTooltip};

const AMI: Asset = asset!("/tests/fixtures/avatar/DATA-Ami-Pena.png");
const ANA: Asset = asset!("/tests/fixtures/avatar/DATA-Ana-Thomas.png");
const DANIELA: Asset = asset!("/tests/fixtures/avatar/DATA-Daniela-Gimenez.png");
const GABRIELA: Asset = asset!("/tests/fixtures/avatar/DATA-Gabriela-Fernandez.png");
const DREW: Asset = asset!("/tests/fixtures/avatar/DATA-Drew-Young.png");
const JIHOO: Asset = asset!("/tests/fixtures/avatar/DATA-Jihoo-Song.png");
const NAM: Asset = asset!("/tests/fixtures/avatar/DATA-Nam-Tran.png");
const ITAI: Asset = asset!("/tests/fixtures/avatar/DATA-Itai-Jordaan.png");
const MARGOT: Asset = asset!("/tests/fixtures/avatar/DATA-Margot-Schroder.png");
const PABLO: Asset = asset!("/tests/fixtures/avatar/DATA-Pablo-Morales.png");
const ROW: &str = "display:flex;align-items:center;flex-wrap:wrap;gap:var(--spacing-4)";

#[component]
fn WrappedStatus(label: String, variant: AvatarStatusDotVariant) -> Element {
    rsx! { AvatarStatusDot { label, variant } }
}

#[component]
pub(super) fn AvatarExamples() -> Element {
    let mut activated = use_signal(|| false);
    let mut status = use_signal(|| 0);
    rsx! {
        section { id: "avatar-examples", "data-astryx-theme": "neutral",
            h2 { "Avatar" }
            h3 { "Showcase" }
            div { style: ROW,
                for (name, asset) in [("Ana Thomas", ANA), ("Drew Young", DREW), ("Jihoo Song", JIHOO), ("Nam Tran", NAM)] {
                    Avatar { key: "{name}", name, src: asset.to_string(), size: AvatarSize::ExtraLarge }
                }
            }
            h3 { "With image" }
            div { style: ROW,
                for (name, asset, size) in [("Ami Pena", AMI, AvatarSize::ExtraSmall), ("Ana Thomas", ANA, AvatarSize::Medium), ("Daniela Gimenez", DANIELA, AvatarSize::Large), ("Gabriela Fernandez", GABRIELA, AvatarSize::ExtraLarge)] {
                    Avatar { key: "{name}", name, src: asset.to_string(), size }
                }
            }
            h3 { "Initials fallback" }
            div { style: ROW,
                for (name, note) in [("John Doe", "First + last"), ("Alice", "Single name"), ("Bob Smith Johnson", "Multi-word"), ("Dr. Sarah Connor", "Prefixed")] {
                    div { key: "{name}", style: "text-align:center", Avatar { name, size: AvatarSize::Large } p { "{note}" } }
                }
            }
            h3 { "Fallback chain" }
            div { style: ROW,
                Avatar { name: "Daniela Gimenez", src: DANIELA.to_string(), size: AvatarSize::Large }
                Avatar { id: "avatar-fallback-image", name: "Invalid User", src: "/missing-avatar-primary.png", fallback_src: AMI.to_string(), size: AvatarSize::Large }
                Avatar { id: "avatar-fallback-initials", name: "Test User", src: "/missing-avatar-primary.png", fallback_src: "/missing-avatar-fallback.png", size: AvatarSize::Large }
                Avatar { id: "avatar-fallback-icon", src: "/missing-avatar-primary.png", size: AvatarSize::Large }
            }
            h3 { "With status" }
            div { style: ROW,
                for (name, asset, label, variant) in [("Itai Jordaan", ITAI, "Online", AvatarStatusDotVariant::SUCCESS), ("Margot Schroder", MARGOT, "Away", AvatarStatusDotVariant::NEUTRAL), ("Pablo Morales", PABLO, "Busy", AvatarStatusDotVariant::ERROR)] {
                    Avatar { key: "{name}", name, src: asset.to_string(), size: AvatarSize::ExtraLarge,
                        status: rsx! { AvatarStatusDot { label, variant } }
                    }
                }
            }
            h3 { "Interactive" }
            div { style: ROW,
                Avatar { name: "Itai Jordaan", src: ITAI.to_string(), size: AvatarSize::ExtraLarge, href: "https://example.com/people/itai-jordaan", target: "_blank", rel: "noopener noreferrer" }
                Avatar { id: "avatar-action", name: "Margot Schroder", src: MARGOT.to_string(), size: AvatarSize::ExtraLarge, on_click: move |_| activated.set(true) }
                if activated() { output { id: "avatar-action-result", "Demo: the profile would open here" } }
            }
            h3 { "Tooltip" }
            div { style: ROW,
                for (name, asset) in [("Ana Thomas", ANA), ("Drew Young", DREW), ("Jihoo Song", JIHOO)] {
                    Avatar { key: "{name}", name, src: asset.to_string(), size: AvatarSize::ExtraLarge }
                }
                Avatar { id: "avatar-tooltip", name: "Itai Jordaan", src: ITAI.to_string(), size: AvatarSize::ExtraLarge, tooltip: AvatarTooltip::Text("Itai Jordaan · Engineering Lead".into()) }
            }
            h3 { "Status changes" }
            button { id: "avatar-status-toggle", onclick: move |_| status.set((status() + 1) % 3), "Change status" }
            Avatar { id: "avatar-with-status", name: "Jane Doe", tooltip: AvatarTooltip::None,
                size: if status() == 0 { AvatarSize::Medium } else { AvatarSize::ExtraLarge },
                status: rsx! { if status() != 2 {
                    WrappedStatus { label: if status() == 0 { "Online" } else { "Away" }, variant: AvatarStatusDotVariant::SUCCESS }
                } }
            }
        }
    }
}
