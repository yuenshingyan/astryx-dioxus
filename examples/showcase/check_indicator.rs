use dioxus::prelude::*;

use astryx_dioxus::{
    CheckIndicator, CheckIndicatorState, HStack, IndicatorSize, SpacingStep, Spinner, SpinnerShade,
    SpinnerSize,
};

#[component]
pub(super) fn CheckIndicatorExamples() -> Element {
    let mut is_selected = use_signal(|| true);
    let mut is_pending = use_signal(|| false);
    let state = if is_selected() {
        CheckIndicatorState::Checked
    } else {
        CheckIndicatorState::Unchecked
    };
    rsx! {
        section { id: "check-indicator-examples", "data-astryx-theme": "neutral",
            h2 { "CheckIndicator" }
            p { "A decorative selection mark. The composing control owns selection and focus." }
            HStack { gap: SpacingStep::Two, "Checked" CheckIndicator { id: "check-mark", state: CheckIndicatorState::Checked, class: "example-check" } }
            HStack { gap: SpacingStep::Two, "Unchecked" CheckIndicator { id: "check-absent", state: CheckIndicatorState::Unchecked } }
            HStack { gap: SpacingStep::Two, "Disabled" CheckIndicator { id: "check-disabled", state: CheckIndicatorState::Checked, size: IndicatorSize::Small, is_disabled: true } }
            HStack { gap: SpacingStep::Two, "Unchecked with replacement"
                CheckIndicator { id: "check-slot", state: CheckIndicatorState::Unchecked,
                    replacement: rsx! { Spinner { size: SpinnerSize::Small, shade: SpinnerShade::Inherit, is_decorative: true } },
                }
            }
            HStack { gap: SpacingStep::Two,
                button {
                    id: "check-example-host",
                    r#type: "button",
                    "aria-pressed": is_selected().to_string(),
                    "aria-busy": is_pending().to_string(),
                    onclick: move |_| is_selected.toggle(),
                    "Select this option"
                    CheckIndicator {
                        id: "check-example-selection",
                        state,
                        size: IndicatorSize::Small,
                        replacement: is_pending().then(|| rsx! {
                            Spinner { size: SpinnerSize::Small, shade: SpinnerShade::Inherit, is_decorative: true }
                        }),
                    }
                }
                label {
                    input {
                        id: "check-example-pending",
                        r#type: "checkbox",
                        checked: is_pending(),
                        onchange: move |event| is_pending.set(event.checked()),
                    }
                    "Show pending indicator"
                }
            }
        }
    }
}
