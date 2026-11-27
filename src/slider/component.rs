use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{FieldLabel, SizeValue};

/// Axis of a single-thumb slider.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderOrientation {
    /// Start-to-end inline axis, following the document direction.
    #[default]
    Horizontal,
    /// Bottom-to-top axis.
    Vertical,
}

/// Presentation of the slider's current value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderValueDisplay {
    /// Do not show a separate value caption.
    #[default]
    None,
    /// Show a caption beside the scale.
    Text,
}

/// A visual tick at a value on the slider's scale.
#[derive(Clone, Debug, PartialEq)]
pub struct SliderMark {
    /// Mark position. Non-finite and out-of-range values are ignored.
    pub value: f64,
    /// Optional visible caption. Marks do not change the slider's step or accessible name.
    pub label: Option<String>,
}

/// A controlled, single-thumb slider using a native range input.
#[component]
pub fn Slider(
    label: String,
    value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default)] orientation: SliderOrientation,
    #[props(default)] marks: Vec<SliderMark>,
    #[props(default)] value_display: SliderValueDisplay,
    /// Formats the controlled value for assistive technology and the optional caption.
    /// Supply step-aligned values so the caption and native thumb agree.
    #[props(default)]
    format_value: Option<Callback<f64, String>>,
    #[props(default)] on_change: Option<EventHandler<f64>>,
    #[props(default)] on_change_end: Option<EventHandler<f64>>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let id = use_hook(|| format!("astryx-slider-{}", current_scope_id().0));
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    if ![value, min, max, step].into_iter().all(f64::is_finite) || min > max || step <= 0.0 {
        return VNode::empty();
    }
    let value = value.clamp(min, max);
    let style = width
        .map(|size| size.as_css().map(|width| format!("inline-size:{width}")))
        .transpose()?;
    let description_id = description.as_ref().map(|_| format!("{id}-description"));
    let mount_id = id.clone();
    let is_vertical = orientation == SliderOrientation::Vertical;
    let marks: Vec<_> = marks
        .into_iter()
        .filter(|mark| (min..=max).contains(&mark.value))
        .collect();
    let has_marks = !marks.is_empty();
    let formatted = format_value
        .map(|format| format.call(value))
        .filter(|text| !text.trim().is_empty());
    let caption = (value_display == SliderValueDisplay::Text)
        .then(|| formatted.clone().unwrap_or_else(|| value.to_string()));
    rsx! {
        div { class: "astryx-slider", style,
            FieldLabel { label, input_id: id.clone(), is_label_hidden, is_disabled,
                description: description.map(|text| rsx! { "{text}" }), description_id: description_id.clone(),
            }
            div { class: "astryx-slider__value-row", "data-caption": caption.is_some().then_some("true"),
            div { class: "astryx-slider__scale", "data-vertical": is_vertical.then_some("true"),
            input { id, class: "astryx-slider__input", r#type: "range", name: html_name, disabled: is_disabled,
                min: min.to_string(), max: max.to_string(), step: step.to_string(), value: value.to_string(),
                "data-value": value.to_string(), "data-vertical": is_vertical.then_some("true"),
                "aria-orientation": is_vertical.then_some("vertical"), "aria-describedby": description_id,
                "aria-valuetext": formatted,
                onmounted: move |_| {
                    if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                    let eval = document::eval(include_str!("sync_slider.js"));
                    let _ = eval.send(mount_id.clone());
                    controller.set(Some(eval));
                },
                oninput: move |event| {
                    if !is_disabled && let Ok(next) = event.value().parse::<f64>() && next.is_finite()
                        && let Some(handler) = on_change { handler.call(next); }
                },
                onchange: move |event| {
                    if !is_disabled && let Ok(next) = event.value().parse::<f64>() && next.is_finite()
                        && let Some(handler) = on_change_end { handler.call(next); }
                },
            }
            if has_marks {
                div { class: "astryx-slider__marks", "aria-hidden": "true",
                    for (index, mark) in marks.into_iter().enumerate() {
                        {
                            let percent = mark_percent(mark.value, min, max);
                            rsx! {
                                span { key: "{index}", class: "astryx-slider__mark", style: "--_slider-mark-position:{percent}%",
                                    "data-filled": (mark.value <= value).then_some("true"),
                                    span { class: "astryx-slider__tick" }
                                    if let Some(label) = mark.label { span { class: "astryx-slider__mark-label", "{label}" } }
                                }
                            }
                        }
                    }
                }
            }
            }
            if let Some(caption) = caption.as_ref() {
                span { class: "astryx-slider__value", "aria-hidden": "true", "{caption}" }
            }
            }
        }
    }
}

fn mark_percent(value: f64, min: f64, max: f64) -> f64 {
    if min == max {
        return 0.0;
    }
    let span = max - min;
    let fraction = if span.is_finite() {
        (value - min) / span
    } else {
        (value / 2.0 - min / 2.0) / (max / 2.0 - min / 2.0)
    };
    fraction * 100.0
}
