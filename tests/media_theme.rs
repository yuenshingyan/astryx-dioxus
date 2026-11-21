use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{MediaTheme, MediaThemeFallback, MediaThemeMode};

#[test]
fn media_mode_has_deterministic_server_output() {
    let html = render_element(
        rsx! { MediaTheme { mode: MediaThemeMode::Auto, fallback: MediaThemeFallback::Light, span { "Media" } } },
    );
    assert!(html.contains("data-astryx-media=\"light\""));
    assert!(html.contains("<span>Media</span>"));
    let html = render_element(rsx! { MediaTheme { mode: MediaThemeMode::Off, span { "Media" } } });
    assert!(html.starts_with("<div") && !html.contains("data-astryx-media="));
}
