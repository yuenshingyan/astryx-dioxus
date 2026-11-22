use dioxus::prelude::*;

use super::collapsible_group_context::{
    CollapsibleGroupContext, CollapsibleGroupDensity, CollapsibleGroupPresentationContext,
    GroupPresentation, GroupSelection,
};

/// Open values and selection mode for a disclosure group.
#[derive(Clone, PartialEq, Eq)]
pub enum CollapsibleGroupValue {
    /// At most one open item; `None` closes all items.
    Single(Option<String>),
    /// Open items toggle independently.
    Multiple(Vec<String>),
}

impl Default for CollapsibleGroupValue {
    fn default() -> Self {
        Self::Single(None)
    }
}

impl CollapsibleGroupValue {
    fn open_values(&self) -> Vec<String> {
        match self {
            Self::Single(value) => value.iter().cloned().collect(),
            Self::Multiple(values) => values.clone(),
        }
    }

    fn toggled(&self, item: String) -> Self {
        match self {
            Self::Single(value) => Self::Single((value.as_ref() != Some(&item)).then_some(item)),
            Self::Multiple(values) if values.contains(&item) => Self::Multiple(
                values
                    .iter()
                    .filter(|value| **value != item)
                    .cloned()
                    .collect(),
            ),
            Self::Multiple(values) => {
                let mut next = values.clone();
                next.push(item);
                Self::Multiple(next)
            }
        }
    }
}

/// Coordinates named Collapsible descendants, without a wrapper unless divided.
#[component]
pub fn CollapsibleGroup(
    children: Element,
    /// Initial mode and open items for uncontrolled use.
    #[props(default)]
    default_value: CollapsibleGroupValue,
    /// Caller-owned open items and mode.
    #[props(default)]
    value: Option<CollapsibleGroupValue>,
    #[props(default)] on_change: Option<EventHandler<CollapsibleGroupValue>>,
    #[props(default)] has_dividers: bool,
    #[props(default)] density: Option<CollapsibleGroupDensity>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut internal = use_signal(|| default_value);
    let is_controlled = value.is_some();
    let current = value.unwrap_or_else(|| internal.read().clone());
    let open_values = current.open_values();
    let toggle = use_callback(move |item| {
        let next = if is_controlled {
            current.toggled(item)
        } else {
            internal.peek().toggled(item)
        };
        if !is_controlled {
            internal.set(next.clone());
        }
        if let Some(handler) = on_change {
            handler.call(next);
        }
    });
    let selection = GroupSelection {
        open_values,
        toggle,
    };
    let mut state =
        use_context_provider(|| CollapsibleGroupContext(Signal::new(selection.clone())));
    if *state.0.peek() != selection {
        state.0.set(selection);
    }
    let density = density.or(has_dividers.then_some(CollapsibleGroupDensity::Balanced));
    let presentation = Some(GroupPresentation {
        has_dividers,
        density,
    });
    let mut chrome =
        use_context_provider(|| CollapsibleGroupPresentationContext(Signal::new(presentation)));
    if *chrome.0.peek() != presentation {
        chrome.0.set(presentation);
    }
    if !has_dividers {
        return children;
    }
    rsx! { div { class: format!("astryx-collapsible-group {class}"), style, "data-density": density.map(CollapsibleGroupDensity::as_str), ..attributes, {children} } }
}
