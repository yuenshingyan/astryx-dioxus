use astryx_dioxus::{Icon, IconDefaultSizeProvider, IconSize};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn icon_is_decorative_by_default_and_named_only_when_meaningful() {
    let html = render(|| rsx! { Icon { icon: rsx! { svg {} } } });
    assert!(html.contains("data-size=\"md\""), "{html}");
    assert!(html.contains("1.25rem"), "{html}");
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
    assert!(!html.contains("role=\"img\""), "{html}");
    let html = render(|| {
        rsx! { Icon {
            icon: rsx! { svg {} }, label: "Success", color: "var(--color-success)"
        } }
    });
    let root = html.split('>').next().expect("root");
    assert!(root.contains("role=\"img\""), "{html}");
    assert!(root.contains("aria-label=\"Success\""), "{html}");
    assert!(!root.contains("aria-hidden="), "{html}");
    assert!(root.contains("color:var(--color-success)"), "{html}");
    let html = render(|| rsx! { Icon { icon: rsx! { svg {} }, label: "  " } });
    assert!(!html.contains("role=\"img\""), "{html}");
}

#[test]
fn icon_size_precedence_is_explicit_then_nearest_provider_then_medium() {
    let html = render(|| {
        rsx! {
            IconDefaultSizeProvider { size: IconSize::Small,
                Icon { icon: rsx! { svg {} } }
                Icon { size: IconSize::Large, icon: rsx! { svg {} } }
                IconDefaultSizeProvider { size: IconSize::ExtraSmall,
                    Icon { icon: rsx! { svg {} } }
                }
                Icon { icon: rsx! { svg {} } }
            }
        }
    });
    assert_eq!(html.matches("data-size=\"sm\"").count(), 2, "{html}");
    for value in [
        "data-size=\"lg\"",
        "data-size=\"xsm\"",
        "1rem",
        "1.5rem",
        "0.75rem",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
}
