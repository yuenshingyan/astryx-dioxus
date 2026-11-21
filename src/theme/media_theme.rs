//! Media surface context adapted from Astryx theme/MediaTheme.tsx.

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::hooks::use_auto_media_mode::use_auto_media_mode;

/// Surface luminance context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaThemeMode {
    /// Light foreground on a dark surface.
    Dark,
    /// Dark foreground on a light surface.
    Light,
    /// Measure the parent surface, falling back when its paint is unknown.
    Auto,
    /// Keep the ambient theme.
    Off,
}

/// First-frame and unknown-surface fallback for automatic mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MediaThemeFallback {
    /// Assume a dark surface.
    #[default]
    Dark,
    /// Assume a light surface.
    Light,
}

impl MediaThemeFallback {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }
}

/// Applies foreground tokens to content without creating a layout box.
#[component]
pub fn MediaTheme(
    /// Surface luminance mode.
    mode: MediaThemeMode,
    /// Fallback used only in automatic mode.
    #[props(default)]
    fallback: MediaThemeFallback,
    /// Content on the surface.
    children: Element,
) -> Element {
    let id = format!("media-theme-{}", current_scope_id().0);
    let fallback = fallback.as_str();
    let mode = match mode {
        MediaThemeMode::Dark => "dark",
        MediaThemeMode::Light => "light",
        MediaThemeMode::Auto => "auto",
        MediaThemeMode::Off => "off",
    };
    let mounted = use_auto_media_mode(id.clone(), mode, fallback);
    let initial = match mode {
        "auto" => Some(fallback),
        "off" => None,
        _ => Some(mode),
    };
    rsx! {
        div { class: "astryx-media-theme", "data-astryx-media": initial,
            "data-astryx-media-owner": id, "data-astryx-media-mode": mode, onmounted: mounted, {children}
        }
    }
}
