use dioxus::core::AttributeValue;
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use crate::{CheckboxIndicator, CheckboxIndicatorState, IndicatorSize, Item, ItemLabel};

/// Controlled checkbox row. The enclosing menu owns keyboard navigation.
#[component]
pub fn DropdownMenuCheckboxItem(
    /// Whether the row is checked.
    value: bool,
    #[props(into)] label: ItemLabel,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] is_disabled: bool,
    /// Requests the opposite value when an enabled row is activated.
    #[props(default)]
    on_change: Option<EventHandler<bool>>,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let state = if value {
        CheckboxIndicatorState::Checked
    } else {
        CheckboxIndicatorState::Unchecked
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
        Attribute::new("role", "menuitemcheckbox", None, false),
        Attribute::new("tabindex", "-1", None, false),
        Attribute::new("aria-checked", value.to_string(), None, false),
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
            if let Some(handler) = on_change {
                handler.call(!value);
            }
            if let Some(handler) = onclick {
                handler.call(event.map(|data| MouseData::from(data)));
            }
        }),
        None,
        false,
    ));
    rsx! {
        Item { label, description, start_content: icon, end_content, style, attributes,
            class: format!("astryx-dropdown-menu-item astryx-dropdown-menu-checkbox-item {class}"),
            marker: rsx! { CheckboxIndicator { state, size: IndicatorSize::Medium, is_disabled, class: "astryx-dropdown-menu-checkbox" } },
        }
    }
}
