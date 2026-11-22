use std::fmt;

use dioxus::core::AttributeValue;
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use super::dropdown_menu_context::DropdownMenuRadioGroupContext;
use crate::{IndicatorSize, Item, ItemLabel, RadioIndicator, RadioIndicatorState};

/// A radio item was rendered outside its required group.
#[derive(Debug)]
struct MissingRadioGroup;

impl fmt::Display for MissingRadioGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DropdownMenuRadioItem requires a DropdownMenuRadioGroup ancestor")
    }
}

impl std::error::Error for MissingRadioGroup {}

/// Controlled radio row. The group owns the value and the menu owns navigation.
#[component]
pub fn DropdownMenuRadioItem(
    value: String,
    #[props(into)] label: ItemLabel,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] is_disabled: bool,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let context =
        try_consume_context::<DropdownMenuRadioGroupContext>().ok_or(MissingRadioGroup)?;
    let settings = (context.0)();
    let is_checked = settings.value.as_deref() == Some(value.as_str());
    let state = if is_checked {
        RadioIndicatorState::Checked
    } else {
        RadioIndicatorState::Unchecked
    };
    let mut attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "role" | "tabindex" | "aria-checked" | "aria-disabled" | "onclick"
            ) && !(is_disabled && attr.name == "onmousedown")
        })
        .collect::<Vec<_>>();
    attributes.extend([
        Attribute::new("role", "menuitemradio", None, false),
        Attribute::new("tabindex", "-1", None, false),
        Attribute::new("aria-checked", is_checked.to_string(), None, false),
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
                return;
            }
            settings.on_change.call(value.clone());
            if let Some(handler) = onclick {
                handler.call(event.map(|data| MouseData::from(data)));
            }
        }),
        None,
        false,
    ));
    rsx! {
        Item { label, description, start_content: icon, end_content, style, attributes,
            class: format!("astryx-dropdown-menu-item astryx-dropdown-menu-radio-item {class}"),
            marker: rsx! { RadioIndicator { state, size: IndicatorSize::Medium, is_disabled, class: "astryx-dropdown-menu-radio" } },
        }
    }
}
