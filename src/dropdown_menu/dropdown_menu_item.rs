use dioxus::core::AttributeValue;
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use crate::{Item, ItemLabel};

/// Foreground treatment of a menu action.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DropdownMenuItemVariant {
    #[default]
    Default,
    Destructive,
}

/// An action row whose focus and keyboard activation are owned by its menu.
#[component]
pub fn DropdownMenuItem(
    #[props(into)] label: ItemLabel,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] is_disabled: bool,
    #[props(default)] variant: DropdownMenuItemVariant,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "role" | "tabindex" | "aria-disabled" | "onclick" | "data-variant"
            ) && !(is_disabled && attr.name == "onmousedown")
        })
        .collect::<Vec<_>>();
    attributes.extend([
        Attribute::new("role", "menuitem", None, false),
        Attribute::new("tabindex", "-1", None, false),
        Attribute::new(
            "data-variant",
            match variant {
                DropdownMenuItemVariant::Default => "default",
                DropdownMenuItemVariant::Destructive => "destructive",
            },
            None,
            false,
        ),
    ]);
    if is_disabled {
        attributes.push(Attribute::new("aria-disabled", "true", None, false));
        attributes.push(Attribute::new(
            "onmousedown",
            AttributeValue::listener(|event: Event<PlatformEventData>| event.prevent_default()),
            None,
            false,
        ));
    }
    attributes.push(Attribute::new(
        "onclick",
        AttributeValue::listener(move |event: Event<PlatformEventData>| {
            if is_disabled {
                event.prevent_default();
            } else if let Some(callback) = onclick {
                callback.call(event.map(|data| MouseData::from(data)));
            }
        }),
        None,
        false,
    ));
    rsx! {
        Item { label, description, start_content: icon, end_content, style, attributes,
            class: format!("astryx-dropdown-menu-item astryx-dropdown-menu-item--action {class}"),
        }
    }
}
