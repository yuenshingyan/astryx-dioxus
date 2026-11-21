use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{BreadcrumbItem, Breadcrumbs, BreadcrumbsVariant, LayerContentBoundary};

#[test]
fn native_landmark_and_caller_name() {
    let html = render_element(
        rsx! { Breadcrumbs { "aria-labelledby": "caption", "data-variant": "caller-value", id: "trail",
            BreadcrumbItem { "Folder" }
        } },
    );
    assert!(
        html.starts_with("<nav") && html.contains("<ol") && html.contains("<li"),
        "{html}"
    );
    assert!(html.contains("aria-labelledby=\"caption\"") && !html.contains("aria-label="));
    assert!(html.contains("id=\"trail\"") && !html.contains("caller-value"));
    assert!(
        render_element(rsx! { Breadcrumbs { BreadcrumbItem { "Folder" } } })
            .contains("aria-label=\"Breadcrumb\"")
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
