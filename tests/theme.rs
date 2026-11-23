use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Theme, ThemeMode, ThemeName};

#[test]
fn system_scope_inside_forced_mode_can_follow_system_again() {
    assert!(ThemeName::new("warm-docs").is_ok());
    assert!(ThemeName::new("warm docs").is_err());
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Theme { theme: ThemeName::NEUTRAL, mode: ThemeMode::Dark,
                Theme { theme: ThemeName::CORE, "Content" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(html.contains("data-astryx-theme=\"neutral\""), "{html}");
    assert!(html.contains("data-theme=\"dark\""), "{html}");
    assert!(html.contains("data-astryx-theme=\"core\""), "{html}");
    assert!(html.contains("color-scheme:light dark"), "{html}");
    assert_eq!(html.matches("data-theme=").count(), 1, "{html}");
}
