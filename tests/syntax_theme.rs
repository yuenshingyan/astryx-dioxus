use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{SyntaxColor, SyntaxTheme, SyntaxThemePreset, SyntaxTokenValue};

#[test]
fn theme_publishes_mode_pairs_and_resets_nested_diff_defaults() {
    let mut theme = SyntaxThemePreset::Dracula.definition();
    theme.colors[0] = SyntaxTokenValue::LightDark {
        light: SyntaxColor::new("rgb(1 2 3 / 50%)").unwrap(),
        dark: SyntaxColor::new("#fff").unwrap(),
    };
    let html = render_element(rsx! {
        SyntaxTheme { theme,
            SyntaxTheme { theme: SyntaxThemePreset::GitHubLight.definition(), "Code" }
        }
    });
    assert!(
        html.contains("data-astryx-syntax-theme=\"dracula\""),
        "{html}"
    );
    assert!(
        html.contains("--color-syntax-keyword:light-dark(rgb(1 2 3 / 50%), #fff);"),
        "{html}"
    );
    assert!(
        html.contains("--color-syntax-background:#ffffff;"),
        "{html}"
    );
    assert_eq!(
        html.matches("--color-syntax-added-marker:var(--color-syntax-punctuation);")
            .count(),
        2
    );
}

#[test]
fn token_rejects_values_that_escape_the_declaration() {
    for value in [
        "",
        "red; color: blue",
        "red, blue",
        "red)",
        "rgb(0 0 0",
        "red/*",
        "red!",
    ] {
        assert!(SyntaxColor::new(value).is_err(), "{value}");
    }
}
