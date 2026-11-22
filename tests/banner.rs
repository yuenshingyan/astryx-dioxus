use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Banner, BannerContainer, BannerStatus};

#[test]
fn custom_identifiers_have_no_implied_icon_or_live_role() {
    assert!(BannerStatus::new("error;").is_err());
    assert!(BannerContainer::new("").is_err());
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Banner { title: "Custom notice", status: BannerStatus::new("custom-status").expect("valid identifier") }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.contains("custom-status") && html.contains("Custom notice"),
        "{html}"
    );
    assert!(
        !html.contains("<svg") && !html.contains("role=\"alert\""),
        "{html}"
    );
}
