use astryx_dioxus::{Spinner, SpinnerShade, SpinnerSize};
use dioxus::events::onclick;
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[derive(Clone, Copy, PartialEq)]
enum LabelMode {
    Absent,
    Text,
    Rich,
}

#[derive(Clone, PartialEq, Props)]
struct CaseProps {
    size: SpinnerSize,
    shade: SpinnerShade,
    label_mode: LabelMode,
    is_decorative: bool,
}

fn case(props: CaseProps) -> Element {
    rsx! {
        Spinner {
            size: props.size, shade: props.shade, is_decorative: props.is_decorative,
            label: (props.label_mode == LabelMode::Text).then(|| "Visible task".to_string()),
            label_content: (props.label_mode == LabelMode::Rich).then(|| rsx! { strong { "Rich task" } }),
        }
    }
}

#[test]
fn all_96_size_shade_content_and_decoration_cases_keep_the_indicator_contract() {
    for (size, name, diameter, stroke) in [
        (SpinnerSize::Small, "sm", 10, 2),
        (SpinnerSize::Medium, "md", 14, 3),
        (SpinnerSize::Large, "lg", 18, 3),
        (SpinnerSize::ExtraLarge, "xl", 28, 4),
    ] {
        for (shade, shade_name) in [
            (SpinnerShade::Default, "default"),
            (SpinnerShade::Subtle, "subtle"),
            (SpinnerShade::OnMedia, "onMedia"),
            (SpinnerShade::Inherit, "inherit"),
        ] {
            for label_mode in [LabelMode::Absent, LabelMode::Text, LabelMode::Rich] {
                for is_decorative in [false, true] {
                    let mut dom = VirtualDom::new_with_props(
                        case,
                        CaseProps {
                            size,
                            shade,
                            label_mode,
                            is_decorative,
                        },
                    );
                    dom.rebuild_in_place();
                    let html = dioxus_ssr::render(&dom);
                    assert!(
                        html.starts_with(if label_mode == LabelMode::Absent {
                            "<span"
                        } else {
                            "<div"
                        }),
                        "{html}"
                    );
                    assert_eq!(
                        html.matches(&format!("data-size=\"{name}\"")).count(),
                        1,
                        "{html}"
                    );
                    assert_eq!(
                        html.matches(&format!("data-shade=\"{shade_name}\""))
                            .count(),
                        1,
                        "{html}"
                    );
                    assert!(
                        html.contains(&format!(
                            "var(--_spinner-box-size, {}px)",
                            diameter + 2 * stroke
                        )),
                        "{html}"
                    );
                    assert_eq!(
                        html.matches(&format!("stroke-width={stroke}")).count(),
                        2,
                        "{html}"
                    );
                    assert_eq!(html.matches("<circle").count(), 2, "{html}");
                    assert_eq!(
                        html.matches("role=\"progressbar\"").count(),
                        usize::from(!is_decorative),
                        "{html}"
                    );
                    assert!(!html.contains("aria-live="), "{html}");
                    assert!(!html.contains("aria-valuenow"), "{html}");
                    if label_mode == LabelMode::Text {
                        assert!(html.contains("astryx-text--weight-bold"), "{html}");
                        assert_eq!(html.contains("aria-labelledby="), !is_decorative, "{html}");
                        assert!(!html.contains("aria-label="), "{html}");
                    }
                    if label_mode == LabelMode::Rich {
                        assert!(html.contains("<strong>Rich task</strong>"), "{html}");
                        assert!(!html.contains("aria-labelledby="), "{html}");
                    }
                }
            }
        }
    }
}

#[test]
fn root_attributes_and_native_callbacks_reach_the_correct_root_without_spoofed_theme_state() {
    let html = render(|| {
        rsx! {
            Spinner { id: "plain", class: "caller", style: "width:999px;height:999px;opacity:0.5", lang: "fr", "data-testid": "plain-test", onmounted: |_| {},
                attributes: vec![onclick(|_| {}), Attribute::new("data-size","xl",None,false), Attribute::new("data-astryx-spinner-id","spoof",None,false)] }
            Spinner { id: "labelled", label: "Task", class: "caller-labelled", style: "width:200px", "data-testid": "labelled-test" }
        }
    });
    assert!(html.starts_with("<span"), "{html}");
    assert!(html.contains("caller"), "{html}");
    assert!(html.contains("lang=\"fr\""), "{html}");
    assert!(html.contains("data-testid=\"plain-test\""), "{html}");
    assert!(html.contains("data-testid=\"labelled-test\""), "{html}");
    assert!(!html.contains("spoof"), "{html}");
    assert!(!html.contains("data-size=\"xl\""), "{html}");
    assert!(
        html.contains("width:999px;height:999px;opacity:0.5;width:var(--_spinner-box-size, 20px)"),
        "{html}"
    );
    assert!(html.contains("style=\"width:200px\""), "{html}");
    assert_eq!(html.matches("data-astryx-spinner-id=").count(), 2, "{html}");
}

#[test]
fn native_name_precedence_and_localized_fallback_work_with_both_label_forms() {
    let html = render(|| {
        rsx! {
            Spinner { label: "Visible", accessible_label: "Alias", "aria-label": "Native" }
            Spinner { loading_label: "Chargement", label_content: rsx! { em { "Patientez" } } }
            Spinner { label: "Caption", "aria-labelledby": "external-name" }
        }
    });
    assert!(html.contains("aria-label=\"Native\""), "{html}");
    assert!(!html.contains("aria-label=\"Alias\""), "{html}");
    assert!(html.contains("aria-label=\"Chargement\""), "{html}");
    assert!(html.contains("<em>Patientez</em>"), "{html}");
    let external = html
        .split("aria-labelledby=\"external-name\"")
        .next()
        .expect("name attribute");
    assert!(
        external
            .rsplit('<')
            .next()
            .expect("indicator")
            .starts_with("span "),
        "{html}"
    );
}

#[test]
fn empty_names_fall_back_and_visible_names_do_not_repeat_or_collide() {
    let html = render(|| {
        rsx! {
            Spinner { label: " ", accessible_label: " " }
            Spinner { label: "First" }
            Spinner { label: "Second" }
        }
    });
    assert_eq!(html.matches("aria-label=\"Loading\"").count(), 1, "{html}");
    let ids: Vec<_> = html
        .split("aria-labelledby=\"")
        .skip(1)
        .map(|part| part.split('"').next().expect("reference"))
        .collect();
    assert_eq!(ids.len(), 2, "{html}");
    assert_ne!(ids[0], ids[1]);
    for id in ids {
        assert_eq!(html.matches(&format!("id=\"{id}\"")).count(), 1, "{html}");
    }
}

#[test]
fn hidden_and_legacy_role_adapters_keep_native_semantics_and_no_local_live_region() {
    let html = render(|| {
        rsx! {
            Spinner { "aria-hidden": "true", loading_label: "Hidden task" }
            Spinner { role: "presentation", label: "Legacy caption" }
        }
    });
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
    assert!(html.contains("role=\"presentation\""), "{html}");
    assert!(!html.contains("aria-live="), "{html}");
    assert!(!html.contains("role=\"status\""), "{html}");
}

#[test]
fn conflicting_labels_and_missing_localized_fallback_are_recoverable_errors() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ErrorBoundary { handle_error: |_| rsx! { output { "Conflicting label rejected" } },
                Spinner { label: "Text", label_content: rsx! { b { "Rich" } } }
            }
            ErrorBoundary { handle_error: |_| rsx! { output { "Missing fallback rejected" } },
                Spinner { loading_label: " " }
            }
        }
    });
    dom.rebuild_in_place();
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Conflicting label rejected"), "{html}");
    assert!(html.contains("Missing fallback rejected"), "{html}");
    assert!(!html.contains("role=\"progressbar\""), "{html}");
}
