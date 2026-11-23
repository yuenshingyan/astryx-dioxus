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

/// A controlled, single-thumb slider using a native range input.
#[component]
pub fn Slider(
    label: String,
    value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default)] orientation: SliderOrientation,
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
    rsx! {
        div { class: "astryx-slider", style,
            FieldLabel { label, input_id: id.clone(), is_label_hidden, is_disabled,
                description: description.map(|text| rsx! { "{text}" }), description_id: description_id.clone(),
            }
            input { id, class: "astryx-slider__input", r#type: "range", name: html_name, disabled: is_disabled,
                min: min.to_string(), max: max.to_string(), step: step.to_string(), value: value.to_string(),
                "data-value": value.to_string(), "data-vertical": is_vertical.then_some("true"),
                "aria-orientation": is_vertical.then_some("vertical"), "aria-describedby": description_id,
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
        }
    }
}
