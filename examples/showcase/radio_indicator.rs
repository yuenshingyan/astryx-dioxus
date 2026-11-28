use dioxus::prelude::*;

use astryx_dioxus::{HStack, IndicatorSize, RadioIndicator, RadioIndicatorState, SpacingStep};

// Unlayered author CSS demonstrates the existing component targets without
// changing the library cascade or pretending to install a theme registry.
const RESTYLE: &str = r#"
  #radio-restyle .astryx-radio-indicator { border-width: 2px; }
  #radio-restyle .astryx-radio-indicator-dot { border-radius: 2px; }
  #radio-restyle .astryx-radio-indicator[data-checked][data-disabled] {
    background-color: var(--color-background-muted);
  }
"#;

#[component]
pub(super) fn RadioIndicatorExamples() -> Element {
    rsx! {
        section { id: "radio-indicator-examples", "data-astryx-theme": "neutral",
            h2 { "RadioIndicator" }
            p { "Decorative states. Hosts supply selection and focus behavior." }
            HStack { id: "radio-indicator-scope", gap: SpacingStep::Two, "data-astryx-indicator-scope": "",
                RadioIndicator { id: "radio-unchecked", state: RadioIndicatorState::Unchecked }
                RadioIndicator { id: "radio-checked", state: RadioIndicatorState::Checked, size: IndicatorSize::Small }
                RadioIndicator { id: "radio-disabled-checked", state: RadioIndicatorState::Checked, is_disabled: true }
            }
            HStack { gap: SpacingStep::Two,
                RadioIndicator { id: "radio-disabled", state: RadioIndicatorState::Unchecked, is_disabled: true }
                RadioIndicator { id: "radio-replacement", state: RadioIndicatorState::Checked, replacement: rsx! { "…" } }
            }
            div { id: "radio-source-example", style: "--radius-full: 0px;",
                h3 { "Source example: checked, medium" }
                RadioIndicator { state: RadioIndicatorState::Checked, size: IndicatorSize::Medium }
            }
            div { id: "radio-restyle",
                h3 { "Restyle an indicator (the common path)" }
                p { "Apply a thicker circle border, square off the dot, and mute the fill when checked and disabled." }
                style { {RESTYLE} }
                HStack { gap: SpacingStep::Two,
                    RadioIndicator { state: RadioIndicatorState::Unchecked }
                    RadioIndicator { state: RadioIndicatorState::Checked }
                    RadioIndicator { state: RadioIndicatorState::Checked, is_disabled: true }
                }
            }
        }
    }
}
