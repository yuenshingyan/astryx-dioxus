use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    CheckboxIndicatorState, CheckboxInput, CheckboxList, CheckboxListItem, Field, FormLayout,
    FormLayoutDirection, FormOptionality, Switch, TextArea, TextInput,
};

#[path = "../examples/showcase/form_layout.rs"]
mod showcase;

use showcase::FormLayoutExamples;

fn opening_tag<'a>(html: &'a str, id: &str) -> &'a str {
    let (before, _) = html
        .split_once(&format!(" id=\"{id}\""))
        .unwrap_or_else(|| panic!("missing {id}: {html}"));
    html[before.rfind('<').unwrap()..]
        .split_once('>')
        .unwrap()
        .0
}

#[test]
fn form_layout_composes_a_nonsemantic_root_in_the_callers_form() {
    let html = render_element(rsx! {
        form { id: "checkout", action: "/save",
            FormLayout {
                id: "outer", class: "caller-layout", style: "max-width:24rem",
                "data-direction": "caller-value", "data-custom": "kept",
                FormLayout { id: "row", direction: FormLayoutDirection::Horizontal,
                    Field { label: "ZIP", input_id: "zip", input { id: "zip", name: "zip" } }
                    button { r#type: "submit", "Save" }
                }
            }
            FormLayout { id: "named", role: "region", "aria-label": "Billing" }
        }
    });
    let outer = opening_tag(&html, "outer");
    assert!(
        outer.starts_with("<div ") && !outer.contains("role="),
        "{outer}"
    );
    for expected in [
        "class=\"astryx-form-layout caller-layout\"",
        "style=\"max-width:24rem\"",
        "data-direction=\"vertical\"",
        "data-custom=\"kept\"",
    ] {
        assert!(outer.contains(expected), "{outer}");
    }
    assert_eq!(outer.matches("data-direction=").count(), 1, "{outer}");
    assert!(
        opening_tag(&html, "row").contains("data-direction=\"horizontal\""),
        "{html}"
    );
    let named = opening_tag(&html, "named");
    assert!(
        named.contains("role=\"region\"") && named.contains("aria-label=\"Billing\""),
        "{named}"
    );
    assert_eq!(html.matches("<form ").count(), 1);
    assert!(
        html.contains("for=\"zip\"") && html.contains("type=\"submit\""),
        "{html}"
    );
}

#[test]
fn form_layout_marks_only_optionality_exceptions() {
    let html = render_element(rsx! {
        FormLayout { default_optionality: FormOptionality::Required,
            TextInput { id: "default-required", label: "Name", value: "" }
            TextInput { id: "explicit-required", label: "Email", value: "", is_required: true }
            TextArea { id: "optional-exception", label: "Notes", value: "", is_optional: true }
            FormLayout { default_optionality: FormOptionality::Optional,
                TextInput { id: "explicit-optional", label: "Nickname", value: "", is_optional: true }
                TextInput { id: "required-exception", label: "Postal code", value: "", is_required: true }
            }
            FormLayout {
                TextInput { id: "unset-required", label: "City", value: "", is_required: true }
                TextInput { id: "unset-optional", label: "County", value: "", is_optional: true }
                TextInput { id: "unset-unmarked", label: "Country", value: "" }
            }
        }
    });
    for (id, marker, aria_required) in [
        ("default-required", None, true),
        ("explicit-required", None, true),
        ("optional-exception", Some("Optional"), false),
        ("explicit-optional", None, false),
        ("required-exception", Some("Required"), true),
        ("unset-required", Some("Required"), true),
        ("unset-optional", Some("Optional"), false),
        ("unset-unmarked", None, false),
    ] {
        let label = html
            .split_once(&format!("for=\"{id}\""))
            .unwrap()
            .1
            .split_once("</label>")
            .unwrap()
            .0;
        assert_eq!(
            label.contains("astryx-field-label__marker"),
            marker.is_some(),
            "{label}"
        );
        if let Some(marker) = marker {
            assert!(label.contains(&format!(">{marker}</span>")), "{label}");
        }
        let control = opening_tag(&html, id);
        assert_eq!(
            control.contains("aria-required=\"true\""),
            aria_required,
            "{control}"
        );
        if id == "default-required" {
            assert!(!control.contains(" required"), "{control}");
        }
    }
}

#[test]
fn form_layout_required_default_does_not_require_binary_answers() {
    let html = render_element(rsx! {
        FormLayout { default_optionality: FormOptionality::Required,
            CheckboxInput { id: "checkbox", label: "Subscribe", value: CheckboxIndicatorState::Unchecked }
            Switch { id: "switch", label: "Enable alerts", value: false }
            CheckboxList { label: "Channels", id: "group",
                CheckboxListItem { label: "Email" }
            }
        }
    });
    let inputs: Vec<_> = html
        .split("<input ")
        .skip(1)
        .map(|input| input.split_once('>').unwrap().0)
        .collect();
    assert_eq!(inputs.len(), 3, "{html}");
    for tag in inputs.into_iter().chain([opening_tag(&html, "group")]) {
        assert!(
            !tag.contains("aria-required") && !tag.contains(" required"),
            "{tag}"
        );
    }
    let required = render_element(rsx! {
        FormLayout { default_optionality: FormOptionality::Optional,
            CheckboxInput { id: "required-checkbox", label: "Accept terms", value: CheckboxIndicatorState::Unchecked, is_required: true }
            Switch { id: "required-switch", label: "Enable security alerts", value: false, is_required: true }
        }
    });
    for id in ["required-checkbox", "required-switch"] {
        let tag = opening_tag(&required, id);
        assert!(
            tag.contains("aria-required=\"true\"") && tag.contains(" required"),
            "{tag}"
        );
    }
}

#[test]
fn form_layout_showcase_renders_the_five_pinned_arrangements() {
    let html = render_element(rsx! { FormLayoutExamples {} });
    for (id, direction) in [
        ("showcase", "vertical"),
        ("nested", "vertical"),
        ("horizontal", "horizontal"),
        ("horizontal-labels", "horizontal-labels"),
        ("mixed-controls", "vertical"),
    ] {
        let root = opening_tag(&html, &format!("form-layout-{id}"));
        assert!(
            root.contains(&format!("data-direction=\"{direction}\"")),
            "{root}"
        );
    }
    let pattern = |id: &str| {
        let (_, rest) = html
            .split_once(&format!("id=\"form-layout-{id}\""))
            .unwrap();
        rest.split("<h3>").next().unwrap()
    };
    for id in ["showcase", "nested"] {
        let address = pattern(id);
        assert_eq!(address.matches("data-direction=\"horizontal\"").count(), 2);
        for value in [
            "Priya",
            "Sharma",
            "priya.sharma@example.com",
            "San Francisco",
            "CA",
            "94105",
        ] {
            assert!(address.contains(&format!("value=\"{value}\"")), "{address}");
        }
    }
    let horizontal = pattern("horizontal");
    for value in ["Jordan", "Rivera"] {
        assert!(
            horizontal.contains(&format!("value=\"{value}\"")),
            "{horizontal}"
        );
    }
    let settings = pattern("horizontal-labels");
    assert_eq!(
        settings
            .matches("data-layout=\"horizontal-labels\"")
            .count(),
        4
    );
    assert!(
        settings.contains("<textarea") && settings.contains("rows=\"3\""),
        "{settings}"
    );
    assert!(
        settings.contains("data-astryx-selector-value=\"America/Los_Angeles\""),
        "{settings}"
    );
    let mixed = pattern("mixed-controls");
    assert!(
        mixed.contains("value=\"Maya Torres\"")
            && mixed.contains("data-astryx-selector-value=\"editor\""),
        "{mixed}"
    );
    let checked: Vec<_> = mixed
        .split("<input ")
        .skip(1)
        .map(|input| input.split_once('>').unwrap().0)
        .filter(|input| input.contains("type=\"checkbox\""))
        .map(|input| input.contains("data-checked=\"true\""))
        .collect();
    assert_eq!(checked, [true, false, true], "{mixed}");
}
