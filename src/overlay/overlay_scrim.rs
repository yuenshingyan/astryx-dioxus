use dioxus::prelude::*;

use crate::{MediaTheme, MediaThemeMode};

/// Backdrop and inherited media theme.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum OverlayScrimMode {
    #[default]
    Dark,
    Light,
    None,
}

/// Portion of the base covered by the overlay.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum OverlayPosition {
    #[default]
    Fill,
    Bottom,
    Top,
}

/// Alignment of content inside the overlay.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum OverlayAlign {
    Start,
    Center,
    #[default]
    End,
}

/// Automatic reveal policy, overridden by explicit open state.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum OverlayShowOn {
    #[default]
    Always,
    Hover,
    Focus,
    HoverOrFocus,
}

#[component]
pub(super) fn OverlayScrim(
    scrim: OverlayScrimMode,
    position: OverlayPosition,
    align: OverlayAlign,
    show_on: OverlayShowOn,
    is_open: Option<bool>,
    children: Element,
) -> Element {
    let (paint, media) = match scrim {
        OverlayScrimMode::Dark => ("dark", MediaThemeMode::Dark),
        OverlayScrimMode::Light => ("light", MediaThemeMode::Light),
        OverlayScrimMode::None => ("none", MediaThemeMode::Off),
    };
    let position = match position {
        OverlayPosition::Fill => "fill",
        OverlayPosition::Bottom => "bottom",
        OverlayPosition::Top => "top",
    };
    let align = match align {
        OverlayAlign::Start => "start",
        OverlayAlign::Center => "center",
        OverlayAlign::End => "end",
    };
    let reveal = match (is_open, show_on) {
        (Some(true), _) | (None, OverlayShowOn::Always) => "open",
        (Some(false), _) => "closed",
        (None, OverlayShowOn::Focus) => "focus",
        (None, OverlayShowOn::Hover | OverlayShowOn::HoverOrFocus) => "hover",
    };
    rsx! {
        div { class: "astryx-overlay-scrim", "data-position": position, "data-align": align,
            "data-scrim": paint, "data-reveal": reveal, inert: (is_open == Some(false)).then_some(true),
            if scrim == OverlayScrimMode::None { {children} }
            else { MediaTheme { mode: media, {children} } }
        }
    }
}
