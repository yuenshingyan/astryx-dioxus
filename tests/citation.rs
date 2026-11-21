use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Citation, CitationSource, CitationVariant};

#[test]
fn link_owns_its_name_role_and_new_tab_attributes() {
    let html = render_element(rsx! {
        Citation { number: 7, variant: CitationVariant::Number, role: "button", "aria-label": "Override",
            source: CitationSource { title: Some("Reference".into()), url: Some("/reference".into()), src: Some("/favicon.png".into()), ..CitationSource::default() },
        }
    });
    for expected in [
        "<a ",
        "role=\"doc-noteref\"",
        "aria-label=\"Citation 7: Reference\"",
        "target=\"_blank\"",
        "rel=\"noopener noreferrer\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        !html.contains("Override") && !html.contains("<img"),
        "{html}"
    );
}

#[test]
fn rejected_destination_is_an_inert_named_reference() {
    let html = render_element(rsx! {
        Citation { number: 2, source: CitationSource { url: Some("data:text/html,untrusted".into()), src: Some("/favicon.png".into()), ..CitationSource::default() } }
    });
    assert!(
        html.starts_with("<span ") && html.contains("role=\"group\"") && !html.contains("href="),
        "{html}"
    );
    assert!(
        html.contains("Citation 2: 2") && html.contains("alt=\"\""),
        "{html}"
    );
}
