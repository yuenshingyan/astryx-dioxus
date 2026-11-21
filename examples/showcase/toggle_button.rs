use dioxus::prelude::*;

use astryx_dioxus::ToggleButton;

#[component]
fn Toggle(
    label: &'static str,
    glyph: &'static str,
    pressed_glyph: &'static str,
    #[props(default)] is_icon_only: bool,
    #[props(default)] color: Option<&'static str>,
) -> Element {
    let mut is_pressed = use_signal(|| false);
    rsx! {
        ToggleButton { label, is_icon_only, is_pressed: is_pressed(),
            icon: rsx! { span { "aria-hidden": "true", "{glyph}" } },
            pressed_icon: rsx! { span { "aria-hidden": "true", style: color.map(|value| format!("color:{value}")), "{pressed_glyph}" } },
            on_pressed_change: move |(next, _)| is_pressed.set(next),
        }
    }
}

#[component]
pub(super) fn ToggleButtonExamples() -> Element {
    let mut is_pressed = use_signal(|| false);
    rsx! {
        section { id: "toggle-button-examples", "data-astryx-theme": "neutral",
            h2 { "ToggleButton" }
            h3 { "Showcase and icon swap" }
            div { style: "display:flex;gap:8px;flex-wrap:wrap",
                Toggle { label: "Favorite", glyph: "☆", pressed_glyph: "★" }
                Toggle { label: "Bookmark", glyph: "♧", pressed_glyph: "♣" }
                Toggle { label: "Notifications", glyph: "♬", pressed_glyph: "♪", is_icon_only: true }
            }
            h3 { "Visible label" }
            ToggleButton { id: "toggle-width-probe", label: "Visible", is_pressed: is_pressed(),
                on_pressed_change: move |(next, _)| is_pressed.set(next),
            }
            h3 { "Pressed icon color" }
            div { style: "display:flex;gap:8px;flex-wrap:wrap",
                Toggle { label: "Bold", glyph: "B", pressed_glyph: "B", color: "var(--color-text-accent)", is_icon_only: true }
                Toggle { label: "Italic", glyph: "𝑰", pressed_glyph: "𝑰", color: "var(--color-text-accent)", is_icon_only: true }
                Toggle { label: "Star", glyph: "☆", pressed_glyph: "★", color: "var(--color-text-accent)", is_icon_only: true }
                Toggle { label: "Like", glyph: "♡", pressed_glyph: "♥", color: "var(--color-error)", is_icon_only: true }
            }
            h3 { "States" }
            div { style: "display:flex;gap:8px;flex-wrap:wrap",
                for (key, is_pressed, is_disabled, is_loading) in [("off", false, false, false), ("on", true, false, false), ("disabled", false, true, false), ("loading", false, false, true)] {
                    div { key: "{key}", "{key} ",
                        ToggleButton { label: "Favorite", is_pressed, is_disabled, is_loading }
                        ToggleButton { label: "Favorite", is_pressed, is_disabled, is_loading, is_icon_only: true,
                            icon: rsx! { span { "aria-hidden": "true", "☆" } },
                            pressed_icon: rsx! { span { "aria-hidden": "true", "★" } },
                        }
                    }
                }
            }
        }
    }
}
