use dioxus::prelude::*;

use astryx_dioxus::{
    CheckboxIndicator, CheckboxIndicatorState, HStack, IndicatorSize, SpacingStep,
};

#[component]
pub(super) fn CheckboxIndicatorExamples() -> Element {
    rsx! {
        section { id: "checkbox-indicator-examples", "data-astryx-theme": "neutral",
            h2 { "CheckboxIndicator" }
            p { "Decorative marks. The host control owns selection and focus." }
            HStack { id: "checkbox-indicator-scope", gap: SpacingStep::Two, "data-astryx-indicator-scope": "",
                CheckboxIndicator { id: "ci-unchecked", state: CheckboxIndicatorState::Unchecked }
                CheckboxIndicator { id: "ci-checked", state: CheckboxIndicatorState::Checked }
                CheckboxIndicator { id: "ci-mixed", state: CheckboxIndicatorState::Indeterminate, size: IndicatorSize::Small }
            }
            HStack { gap: SpacingStep::Two,
                CheckboxIndicator { id: "ci-disabled", state: CheckboxIndicatorState::Unchecked, is_disabled: true }
                CheckboxIndicator { id: "ci-replacement", state: CheckboxIndicatorState::Checked, replacement: rsx! { "…" } }
                CheckboxIndicator { state: CheckboxIndicatorState::Checked, style: "border-width:2px;border-radius:50%" }
            }
        }
    }
}
