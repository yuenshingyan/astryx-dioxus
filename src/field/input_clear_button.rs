use dioxus::prelude::*;

use crate::{Button, ButtonSize, ButtonVariant, Icon, IconSize};

/// Field-owned clear affordance. The input owner supplies the change and focus behavior.
#[component]
pub(crate) fn InputClearButton(label: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Button { class: "astryx-input-clear-button", label, tooltip: "Clear", is_icon_only: true,
            variant: ButtonVariant::Ghost, size: ButtonSize::Small, onclick,
            onmousedown: move |event: MouseEvent| event.prevent_default(),
            icon: rsx! {
                Icon { size: IconSize::Small, class: "astryx-input-clear-icon", icon: rsx! {
                    svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round",
                        path { d: "M6 6l12 12M18 6L6 18" }
                    }
                } }
            }
        }
    }
}
