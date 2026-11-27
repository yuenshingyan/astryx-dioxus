use dioxus::prelude::*;

use astryx_dioxus::{Slider, SliderMark, SliderOrientation, SliderValueDisplay};

#[component]
pub(super) fn SliderExamples() -> Element {
    let mut volume = use_signal(|| 50.0);
    let mut brightness = use_signal(|| 25.0);
    rsx! {
        section { id: "slider-examples",
            h2 { "Slider" }
            form {
                Slider { label: "Volume", value: volume(), html_name: "volume",
                    value_display: SliderValueDisplay::Text, format_value: |value: f64| format!("{value}%"),
                    width: Some(300.into()), marks: [0, 25, 50, 75, 100].into_iter()
                        .map(|value| SliderMark { value: f64::from(value), label: Some(value.to_string()) }).collect::<Vec<_>>(),
                    on_change: move |value| volume.set(value) }
                Slider { label: "Brightness", value: brightness(), orientation: SliderOrientation::Vertical,
                    step: 5.0, marks: vec![SliderMark { value: 0.0, label: None }, SliderMark { value: 100.0, label: None }], on_change: move |value| brightness.set(value) }
                Slider { label: "Fixed volume", value: 30.0 }
                Slider { label: "Unavailable volume", value: 40.0, html_name: "unavailable", is_disabled: true }
            }
        }
    }
}
