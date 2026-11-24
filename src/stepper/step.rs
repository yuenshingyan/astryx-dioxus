use std::cmp::Ordering;

use dioxus::prelude::*;

use super::stepper_context::StepperContext;
use crate::VisuallyHidden;

/// Built-in visual indicator for a step.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepIndicatorPreset {
    /// Show a check for a completed step, otherwise its number.
    #[default]
    Auto,
    /// Always show the step number.
    Number,
    /// Omit the indicator.
    None,
}

/// One step in an ordered progress list. Indices are zero-based.
#[component]
pub fn Step(
    step: usize,
    label: String,
    #[props(default)] description: Option<String>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] children: Element,
    #[props(default)] indicator: StepIndicatorPreset,
    #[props(default)] is_optional: bool,
    #[props(default = "Optional".to_owned())] optional_label: String,
    #[props(default = "Completed".to_owned())] completed_label: String,
    #[props(default)] is_disabled: bool,
) -> Element {
    let Some(ctx) = try_consume_context::<StepperContext>() else {
        return VNode::empty();
    };
    let settings = (ctx.0)();
    let progress = step.cmp(&settings.active_step);
    let phase = match progress {
        Ordering::Less => "completed",
        Ordering::Equal => "current",
        Ordering::Greater => "upcoming",
    };
    let marker = if progress == Ordering::Less && indicator == StepIndicatorPreset::Auto {
        "✓".to_owned()
    } else {
        step.saturating_add(1).to_string()
    };
    let summary = rsx! {
        if indicator != StepIndicatorPreset::None {
            span { class: "astryx-step__indicator", "aria-hidden": "true", "{marker}" }
        }
        span { class: "astryx-step__label", "{label}" }
        if is_optional { span { class: "astryx-step__optional", "{optional_label}" } }
        if progress == Ordering::Less { VisuallyHidden { "{completed_label}" } }
    };
    rsx! {
        li { class: "astryx-step", "data-progress": phase,
            "data-disabled": is_disabled.then_some("true"),
            "aria-current": (progress == Ordering::Equal).then_some("step"),
            div { class: "astryx-step__row",
                if let Some(select) = settings.on_step_click {
                    button { class: "astryx-step__summary", r#type: "button",
                        "aria-disabled": is_disabled.then_some("true"),
                        onclick: move |_| { if !is_disabled { select.call(step); } },
                        {summary}
                    }
                } else {
                    div { class: "astryx-step__summary", {summary} }
                }
                {end_content}
            }
            if let Some(description) = description { div { class: "astryx-step__description", "{description}" } }
            {children}
        }
    }
}
