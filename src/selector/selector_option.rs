use dioxus::prelude::*;

use crate::{Item, ItemLabel, ItemLayout};

/// Presentational content for a selector option or selected value.
/// The selector, not this row, owns selection and keyboard interaction.
#[component]
pub fn SelectorOption(
    /// Primary option text or rich inline content.
    #[props(into)]
    label: ItemLabel,
    /// Supporting content after or below the label.
    #[props(default)]
    description: Option<ItemLabel>,
    /// Caller-rendered leading artwork.
    #[props(default)]
    icon: Option<Element>,
    /// Arrangement of the label and description.
    #[props(default)]
    layout: ItemLayout,
    /// Caller-rendered trailing content.
    #[props(default)]
    end_content: Option<Element>,
    /// Additional classes on the Item root.
    #[props(default)]
    class: String,
    /// Additional inline declarations on the Item root.
    #[props(default)]
    style: String,
) -> Element {
    rsx! {
        Item { label, description, start_content: icon, layout, end_content,
            class: format!("astryx-selector-option {class}"), style,
        }
    }
}
