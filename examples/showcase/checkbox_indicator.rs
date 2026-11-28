use dioxus::prelude::*;

use astryx_dioxus::{
    CheckIndicator, CheckIndicatorState, CheckboxIndicator, CheckboxIndicatorState, CheckboxInput,
    HStack, IndicatorSize, RadioIndicator, SpacingStep, Spinner, SpinnerShade, SpinnerSize,
};

const RESTYLE_CHECKBOX: &str = r#"
#ci-restyle .astryx-checkbox-indicator {
  border-radius: var(--radius-full);
  border-width: 2px;
}
#ci-restyle .astryx-checkbox-indicator[data-checked="checked"] {
  background-color: var(--color-accent);
  border-color: var(--color-accent);
}
#ci-restyle .astryx-checkbox-indicator[data-checked="checked"][data-disabled] {
  background-color: var(--color-background-muted);
}
"#;

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
                CheckboxIndicator { id: "ci-pending", state: CheckboxIndicatorState::Checked, replacement: pending_spinner() }
            }
            NativeOwnerExample {}
            h3 { "Restyle an indicator (the common path)" }
            style { {RESTYLE_CHECKBOX} }
            HStack { id: "ci-restyle", gap: SpacingStep::Two,
                CheckboxIndicator { id: "ci-brand-unchecked", state: CheckboxIndicatorState::Unchecked }
                CheckboxIndicator { id: "ci-brand-checked", state: CheckboxIndicatorState::Checked }
                CheckboxIndicator { id: "ci-brand-disabled", state: CheckboxIndicatorState::Checked, is_disabled: true }
            }
            h3 { "Replace an indicator with your own component" }
            p { "Compose BrandCheckbox directly. Pending content takes precedence over the star." }
            HStack { gap: SpacingStep::Two,
                BrandCheckbox { id: "ci-star", state: CheckboxIndicatorState::Checked }
                BrandCheckbox { id: "ci-star-pending", state: CheckboxIndicatorState::Checked, replacement: pending_spinner() }
            }
            h3 { "Use radio visuals for single selection" }
            p { "Unchecked, then checked: a radio visual can replace the single-selection check mark. The three-state checkbox is unchanged." }
            HStack { gap: SpacingStep::Two,
                span { "Check" }
                span { CheckIndicator { state: CheckIndicatorState::Unchecked } }
                CheckIndicator { state: CheckIndicatorState::Checked }
                span { "Radio" }
                RadioIndicator { state: CheckIndicatorState::Unchecked }
                RadioIndicator { state: CheckIndicatorState::Checked }
            }
        }
    }
}

#[component]
fn NativeOwnerExample() -> Element {
    let mut state = use_signal(|| CheckboxIndicatorState::Unchecked);
    rsx! {
        CheckboxInput {
            id: "ci-native-owner",
            label: "Show completed tasks",
            value: state(),
            on_change: move |is_checked| state.set(if is_checked {
                CheckboxIndicatorState::Checked
            } else {
                CheckboxIndicatorState::Unchecked
            }),
        }
    }
}

fn pending_spinner() -> Element {
    rsx! { Spinner { size: SpinnerSize::Small, shade: SpinnerShade::Inherit, is_decorative: true } }
}

#[component]
fn BrandCheckbox(
    id: &'static str,
    state: CheckboxIndicatorState,
    #[props(default)] size: IndicatorSize,
    #[props(default)] is_disabled: bool,
    #[props(default)] replacement: Option<Element>,
) -> Element {
    let dimension = match size {
        IndicatorSize::Small => 20,
        IndicatorSize::Medium => 24,
    };
    let opacity = if is_disabled { 0.5 } else { 1.0 };
    rsx! {
        span {
            id,
            "aria-hidden": "true",
            style: "box-sizing:border-box;display:grid;place-items:center;width:{dimension}px;height:{dimension}px;border-radius:var(--radius-inner);border:var(--border-width) solid var(--color-border-emphasized);color:var(--color-accent);opacity:{opacity}",
            if let Some(replacement) = replacement {
                {replacement}
            } else if state == CheckboxIndicatorState::Checked {
                span { "★" }
            }
        }
    }
}
