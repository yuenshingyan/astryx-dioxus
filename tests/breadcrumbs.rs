use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{BreadcrumbItem, Breadcrumbs, BreadcrumbsVariant, LayerContentBoundary};

#[test]
fn native_landmark_and_labelledby_override() {
    let html = render_element(
        rsx! { Breadcrumbs { label: "Folder trail", "aria-labelledby": "caption", "data-variant": "caller-value", id: "trail",
            BreadcrumbItem { "Folder" }
        } },
    );
    assert!(
        html.starts_with("<nav") && html.contains("<ol") && html.contains("<li"),
        "{html}"
    );
    assert!(html.contains("aria-labelledby=\"caption\"") && !html.contains("aria-label="));
    assert!(html.contains("id=\"trail\"") && !html.contains("caller-value"));
}

#[test]
fn landmark_label_falls_back_unless_the_caller_names_it() {
    let default = render_element(rsx! { Breadcrumbs { BreadcrumbItem { "Folder" } } });
    let explicit =
        render_element(rsx! { Breadcrumbs { label: "Folder trail", BreadcrumbItem { "Folder" } } });
    let overridden = render_element(rsx! {
        Breadcrumbs { label: "Folder trail", "aria-label": "Project location",
            BreadcrumbItem { "Folder" }
        }
    });

    for (html, expected) in [
        (default, "Breadcrumb"),
        (explicit, "Folder trail"),
        (overridden, "Project location"),
    ] {
        let (nav, _) = html.split_once('>').expect("opening nav tag");
        assert!(nav.starts_with("<nav "), "{html}");
        assert_eq!(nav.matches("aria-label=").count(), 1, "{html}");
        assert!(
            nav.contains(&format!("aria-label=\"{expected}\"")),
            "{html}"
        );
    }
}

#[test]
fn custom_separator_is_decorative_without_hiding_crumb_content() {
    let html = render_element(rsx! {
        Breadcrumbs { separator: rsx! { span { "Next level" } },
            BreadcrumbItem { href: "/projects", "Projects" }
            BreadcrumbItem { is_current: true, "Details" }
        }
    });

    assert_eq!(html.matches("Next level").count(), 2, "{html}");
    assert_eq!(
        html.matches("aria-hidden=\"true\"><span>Next level</span></span>")
            .count(),
        2,
        "{html}"
    );
    assert!(html.contains("href=\"/projects\">Projects</a>"), "{html}");
    assert!(
        html.contains("aria-current=\"page\">Details</span>"),
        "{html}"
    );
}

#[test]
fn layer_resets_separator_and_variant() {
    let html = render_element(
        rsx! { Breadcrumbs { variant: BreadcrumbsVariant::SUPPORTING, separator: rsx! { "Custom separator" },
            BreadcrumbItem { "Folder" }
            LayerContentBoundary { BreadcrumbItem { "Layer item" } }
        } },
    );
    assert_eq!(
        html.matches("data-variant=\"supporting\"").count(),
        2,
        "{html}"
    );
    assert_eq!(
        html.matches("data-variant=\"default\"").count(),
        1,
        "{html}"
    );
    assert_eq!(html.matches("Custom separator").count(), 1, "{html}");
    assert!(html.contains("astryx-breadcrumb-item__slash"));
    assert!(BreadcrumbsVariant::new("invalid name").is_err());
}
