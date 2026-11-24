use dioxus::prelude::*;

use crate::{ButtonSize, ButtonVariant, DropdownMenu, LayerPlacement};

/// An icon-only trigger for a dropdown of caller-provided actions.
#[component]
pub fn MoreMenu(
    children: Element,
    #[props(default = "More options".to_owned())] label: String,
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default)] size: Option<ButtonSize>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] is_disabled: bool,
    #[props(default = LayerPlacement::Below)] placement: LayerPlacement,
) -> Element {
    let icon = icon.unwrap_or_else(|| rsx! {
        svg { view_box: "0 0 24 24", width: "1em", height: "1em", fill: "currentColor", "aria-hidden": "true",
            for x in [5, 12, 19] {
                circle { cx: "{x}", cy: "12", r: "2" }
            }
        }
    });
    rsx! {
        DropdownMenu { label: label.clone(), placement, is_disabled, trigger_icon: icon,
            is_trigger_icon_only: true, trigger_variant: variant, trigger_size: size, trigger_tooltip: label,
            {children}
        }
    }
}
