use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{CheckboxIndicatorState, CheckboxInput, FieldStatusInput, InputStatusType};

#[test]
fn mixed_checkbox_renders_native_label_and_description_relationships() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            CheckboxInput { id: "alerts", label: "Alerts", description: "Some alerts enabled", value: CheckboxIndicatorState::Indeterminate }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "type=\"checkbox\"",
        "id=\"alerts\"",
        "for=\"alerts\"",
        "aria-checked=\"mixed\"",
        "aria-describedby=\"alerts-desc\"",
        "id=\"alerts-desc\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}

#[test]
fn row_hover_scope_depends_on_disabled_not_loading() {
    for (is_disabled, is_loading) in [(false, false), (false, true), (true, false), (true, true)] {
        let mut dom = VirtualDom::new_with_props(
            |(is_disabled, is_loading)| {
                rsx! {
                    CheckboxInput { label: "Backups", html_name: "backups", value: CheckboxIndicatorState::Checked,
                        is_disabled, is_loading,
                    }
                }
            },
            (is_disabled, is_loading),
        );
        dom.rebuild_in_place();
        let html = render(&dom);
        let (_, row) = html
            .split_once("<div class=\"astryx-checkbox-input__row\"")
            .unwrap();
        let (row_attrs, _) = row.split_once('>').unwrap();
        let (_, input) = html.split_once("<input ").unwrap();
        let (input_attrs, _) = input.split_once('>').unwrap();
        assert_eq!(
            row_attrs.contains(" data-astryx-indicator-scope"),
            !is_disabled,
            "{html}"
        );
        assert_eq!(
            html.matches("data-astryx-indicator-scope").count(),
            usize::from(!is_disabled),
            "{html}"
        );
        assert_eq!(
            input_attrs.contains(" aria-disabled=\"true\""),
            is_disabled,
            "{html}"
        );
        assert_eq!(input_attrs.contains(" form=\"\""), is_disabled, "{html}");
        assert_eq!(
            input_attrs.contains(" name=\"backups\""),
            !is_disabled,
            "{html}"
        );
        assert!(!input_attrs.contains(" disabled"), "{html}");
        assert_eq!(
            input_attrs.contains(" aria-busy=\"true\""),
            is_loading,
            "{html}"
        );
        assert_eq!(html.contains("astryx-spinner"), is_loading, "{html}");
    }
}

#[test]
fn explicit_required_preserves_native_validation_and_reports_status() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            form {
                CheckboxInput { label: "Terms", html_name: "terms", value: CheckboxIndicatorState::Unchecked, is_required: true }
                CheckboxInput { label: "Updates", html_name: "updates", value: CheckboxIndicatorState::Unchecked }
                CheckboxInput { label: "Policy", html_name: "policy", value: CheckboxIndicatorState::Unchecked, is_required: true,
                    status: FieldStatusInput { status_type: InputStatusType::Error, message: Some("Accept the policy".to_owned()), message_id: None },
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let input = |name| {
        html.split("<input ")
            .skip(1)
            .filter_map(|part| part.split_once('>').map(|(attrs, _)| attrs))
            .find(|attrs| attrs.contains(&format!(" name=\"{name}\"")))
            .unwrap_or_else(|| panic!("missing input named {name}: {html}"))
    };

    for (name, is_invalid) in [("terms", false), ("policy", true)] {
        let attrs = input(name);
        assert!(attrs.contains("type=\"checkbox\""), "{attrs}");
        assert!(attrs.contains(" required"), "{attrs}");
        assert!(attrs.contains(" aria-required=\"true\""), "{attrs}");
        assert!(
            attrs.contains(&format!(" aria-invalid=\"{is_invalid}\"")),
            "{attrs}"
        );
        assert!(!attrs.contains(" form="), "{attrs}");
    }

    let attrs = input("updates");
    for unexpected in [" required", " aria-required", " aria-invalid"] {
        assert!(
            !attrs.contains(unexpected),
            "unexpected {unexpected}: {attrs}"
        );
    }
}
