use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::SideNavSection;

#[test]
fn hidden_group_preserves_its_title_and_rejects_name_overrides() {
    let html = render_element(rsx! {
        SideNavSection { title: "Account", subtitle: "Production", is_header_hidden: true,
            id: "account", role: "menu", "aria-labelledby": "incorrect", "aria-label": "incorrect",
            end_content: rsx! { span { "2" } },
            a { href: "/profile", "Profile" }
        }
    });
    let label = html
        .split("aria-labelledby=\"")
        .nth(1)
        .and_then(|tail| tail.split('"').next())
        .expect("group name reference");
    assert!(html.contains(&format!("id=\"{label}\"")));
    for expected in [
        "role=\"group\"",
        "id=\"account\"",
        "clip:rect(0,0,0,0)",
        "Account",
        "Production",
        "Profile",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        !html.contains("incorrect") && !html.contains("role=\"menu\""),
        "{html}"
    );
}
