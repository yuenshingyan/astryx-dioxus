use dioxus::prelude::*;
use dioxus_ssr::{render, render_element};

use astryx_dioxus::{Step, StepIndicatorPreset, Stepper};

#[test]
fn static_optional_step_appends_the_default_cue_with_a_decorative_separator() {
    let html = render_element(rsx! {
        Stepper { label: "Setup progress", active_step: 0_usize,
            Step { step: 0_usize, label: "Connect a repository", is_optional: true,
                indicator: StepIndicatorPreset::None,
            }
        }
    });
    assert!(!html.contains("<button"), "{html}");
    let label = html.find("Connect a repository").unwrap();
    let separator = html.find("<span aria-hidden=\"true\">•</span>").unwrap();
    let cue = html.find("Optional").unwrap();
    assert!(label < separator && separator < cue, "{html}");
}

#[test]
fn clickable_optional_step_keeps_caller_text_in_the_button_name_from_content() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Stepper { label: "Checkout progress", active_step: 0_usize, on_step_click: |_| {},
                Step { step: 1_usize, label: "Review", is_optional: true, optional_label: "Facultatif" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let (_, rest) = html.split_once("<button").unwrap();
    let (button, _) = rest.split_once("</button>").unwrap();
    let (attributes, content) = button.split_once('>').unwrap();
    assert!(!attributes.contains("aria-label"), "{button}");
    assert!(content.contains("Review</span>"), "{button}");
    assert!(
        content.contains("<span aria-hidden=\"true\">•</span>Facultatif"),
        "{button}"
    );
    assert!(!html.contains("Optional"), "{html}");
}

#[test]
fn unmarked_steps_and_blank_overrides_do_not_render_an_optional_cue() {
    let html = render_element(rsx! {
        Stepper { label: "Setup progress", active_step: 0_usize,
            Step { step: 0_usize, label: "Account", optional_label: "Facultatif" }
            Step { step: 1_usize, label: "Review", is_optional: true, optional_label: "  " }
        }
    });
    assert!(!html.contains("Facultatif"), "{html}");
    assert!(!html.contains("Optional"), "{html}");
    assert!(!html.contains('•'), "{html}");
}
