use dioxus::prelude::*;

use astryx_dioxus::{Badge, BadgeVariant, Icon, IconSize, Token, TokenColor};

const ROW: &str = "display:flex;gap:var(--spacing-2);flex-wrap:wrap";

#[component]
fn DemoIcon(path: &'static str) -> Element {
    rsx! { Icon { size: IconSize::Small, icon: rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", path { d: path } }
    } } }
}

#[component]
pub(super) fn TokenExamples() -> Element {
    const TAG: &str = "M3 4h9l9 9-8 8-10-10Zm5 3h.01";
    let mut message = use_signal(|| "");
    let colors = [
        ("Default", TokenColor::DEFAULT),
        ("Red", TokenColor::RED),
        ("Orange", TokenColor::ORANGE),
        ("Yellow", TokenColor::YELLOW),
        ("Green", TokenColor::GREEN),
        ("Teal", TokenColor::TEAL),
        ("Cyan", TokenColor::CYAN),
        ("Blue", TokenColor::BLUE),
        ("Purple", TokenColor::PURPLE),
        ("Pink", TokenColor::PINK),
        ("Gray", TokenColor::GRAY),
    ];
    rsx! {
        section { id: "token-examples", "data-astryx-theme": "neutral",
            h2 { "Token" }
            h3 { "Showcase" }
            div { style: ROW,
                Token { label: "Default" }
                Token { label: "Removable", color: TokenColor::BLUE, on_remove: |_| {} }
                Token { label: "Design", color: TokenColor::PURPLE, icon: rsx! { DemoIcon { path: TAG } } }
            }
            h3 { "Colors" }
            for is_disabled in [false, true] {
                div { key: "{is_disabled}", style: ROW,
                    for (label, color) in colors.iter().cloned() { Token { key: "{label}", label, color, is_disabled } }
                }
            }
            h3 { "Icons" }
            div { style: ROW,
                for (label, color, path) in [
                    ("Sarah Chen", TokenColor::BLUE, "M4 21v-2a8 8 0 0 1 16 0v2M16 7a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"),
                    ("Featured", TokenColor::YELLOW, "m12 2 3 7 8 1-6 5 2 8-7-4-7 4 2-8-6-5 8-1Z"),
                    ("Design", TokenColor::PURPLE, TAG),
                    ("Verified", TokenColor::GREEN, "m12 2 8 4v5c0 5-3 8-8 11-5-3-8-6-8-11V6Zm-4 10 3 3 5-5"),
                ] { Token { key: "{label}", label, color, icon: rsx! { DemoIcon { path } } } }
            }
            h3 { "End content" }
            div { style: ROW,
                Token { label: "Inbox", color: TokenColor::BLUE, end_content: rsx! { Badge { label: 12, variant: BadgeVariant::INFO } } }
                Token { label: "Reviews", color: TokenColor::PURPLE, end_content: rsx! { Badge { label: 3, variant: BadgeVariant::PURPLE } } }
                Token { label: "Resolved", color: TokenColor::GREEN, end_content: rsx! { Badge { label: "Done", variant: BadgeVariant::SUCCESS } } }
            }
            h3 { "Removable" }
            for (group, items) in [
                ("Filters", [("Status: Open", TokenColor::GREEN), ("Priority: High", TokenColor::RED), ("Team: Design", TokenColor::PURPLE)]),
                ("Assignees", [("Sarah Chen", TokenColor::BLUE), ("Alex Rivera", TokenColor::BLUE), ("Jordan Lee", TokenColor::BLUE)]),
            ] {
                div { key: "{group}", p { "{group}" } div { style: ROW,
                    for (label, color) in items { Token { key: "{label}", label, color, on_remove: |_| {} } }
                } }
            }
            h3 { "Clickable" }
            div { style: ROW,
                for (label, color) in [("Bug", TokenColor::RED), ("Feature", TokenColor::BLUE), ("Enhancement", TokenColor::GREEN), ("Documentation", TokenColor::GRAY)] {
                    Token { key: "{label}", label, color, on_click: move |_| message.set(label) }
                }
            }
            h3 { "Action and link contracts" }
            div { style: ROW,
                Token { id: "token-action", label: "Active", on_click: move |_| message.set("Activated"), on_remove: move |_| message.set("Removed"),
                    icon: rsx! { DemoIcon { path: TAG } }, end_content: rsx! { button { onclick: move |_| message.set("Nested"), "Count" } } }
                Token { id: "token-disabled", label: "Disabled", is_disabled: true, on_click: move |_| message.set("Unexpected"), on_remove: move |_| message.set("Unexpected") }
                Token { id: "token-link", label: "Destination", href: "#token-examples", on_click: move |_| message.set("Navigated"), on_remove: move |_| message.set("Removed link") }
            }
            output { id: "token-message", "{message}" }
        }
    }
}
