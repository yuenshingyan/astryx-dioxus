use dioxus::prelude::*;

use astryx_dioxus::{Slider, SliderOrientation};

#[component]
pub(super) fn SliderExamples() -> Element {
    let mut volume = use_signal(|| 50.0);
    let mut brightness = use_signal(|| 25.0);
    rsx! {
        section { id: "slider-examples",
            h2 { "Slider" }
            form {
                Slider { label: "Volume", value: volume(), html_name: "volume",
                    width: Some(300.into()), on_change: move |value| volume.set(value) }
                Slider { label: "Brightness", value: brightness(), orientation: SliderOrientation::Vertical,
                    step: 5.0, on_change: move |value| brightness.set(value) }
                Slider { label: "Fixed volume", value: 30.0 }
                Slider { label: "Unavailable volume", value: 40.0, html_name: "unavailable", is_disabled: true }
            }
        }
    }
}
