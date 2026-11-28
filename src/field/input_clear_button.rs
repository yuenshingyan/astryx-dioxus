use dioxus::prelude::*;

use crate::i18n::use_message;
use crate::{Button, ButtonSize, ButtonVariant, Icon, IconSize};

/// Field-owned clear affordance. The caller owns the contextual name, change, and focus return.
#[component]
pub(crate) fn InputClearButton(label: String, onclick: EventHandler<MouseEvent>) -> Element {
    let tooltip = use_message("@astryx.inputClearButton.tooltip", "Clear");
    rsx! {
        Button { class: "astryx-input-clear-button", label, tooltip, is_icon_only: true,
            variant: ButtonVariant::Ghost, size: ButtonSize::Small, onclick,
            onpointerdown: move |event: PointerEvent| {
                // Cancelling touch pointerdown can drop the click in WebKit. Leave it
                // alone and use the mouse default guard for the compatibility event.
                if event.pointer_type() != "touch" {
                    event.prevent_default();
                }
            },
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
