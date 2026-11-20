use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{FieldStatus, FieldStatusVariant, InputStatusType};

#[test]
fn visible_message_does_not_own_a_live_region() {
    let html = render_element(rsx! { FieldStatus {
        id: "validation", r#type: InputStatusType::Error, message: "Enter a name"
    } });
    for expected in [
        "id=\"validation\"",
        "data-type=\"error\"",
        "data-variant=\"attached\"",
        "Enter a name",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in ["<svg", "role=", "aria-live="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn detached_glyph_is_decorative_and_custom_variants_are_plain() {
    let detached = render_element(rsx! { FieldStatus {
        r#type: InputStatusType::Success, message: "Saved", variant: FieldStatusVariant::DETACHED
    } });
    assert!(
        detached.contains("astryx-field-status-icon") && detached.contains("aria-hidden=\"true\"")
    );
    for name in ["tooltip", "custom"] {
        let variant = FieldStatusVariant::new(name).unwrap();
        let html = render_element(rsx! { FieldStatus {
            r#type: InputStatusType::Warning, message: "Review", variant
        } });
        assert!(html.contains(&format!("data-variant=\"{name}\"")) && !html.contains("<svg"));
    }
    assert!(FieldStatusVariant::new("bad variant").is_err());
}
