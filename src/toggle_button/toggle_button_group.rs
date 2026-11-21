use dioxus::prelude::*;

use crate::{ButtonGroupOrientation, ButtonSize};

/// Controlled selection and its matching change callback.
#[derive(Clone, PartialEq)]
pub enum ToggleButtonGroupSelection {
    /// At most one selected value. Activating it again clears selection.
    Single {
        /// Currently selected value.
        value: Option<String>,
        /// Receives the next value, or `None` when cleared.
        onchange: EventHandler<Option<String>>,
    },
    /// Independently selectable values.
    Multiple {
        /// Currently selected values in selection order.
        values: Vec<String>,
        /// Receives the next ordered selection.
        onchange: EventHandler<Vec<String>>,
    },
}

impl ToggleButtonGroupSelection {
    pub(super) fn contains(&self, item: &str) -> bool {
        match self {
            Self::Single { value, .. } => value.as_deref() == Some(item),
            Self::Multiple { values, .. } => values.iter().any(|value| value == item),
        }
    }

    pub(super) fn toggle(&self, item: &str) {
        match self {
            Self::Single { value, onchange } => {
                onchange.call((value.as_deref() != Some(item)).then(|| item.to_owned()))
            }
            Self::Multiple { values, onchange } => {
                let mut next = values.clone();
                if self.contains(item) {
                    next.retain(|value| value != item);
                } else {
                    next.push(item.to_owned());
                }
                onchange.call(next);
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub(super) struct GroupSettings {
    pub selection: ToggleButtonGroupSelection,
    pub size: Option<ButtonSize>,
    pub is_disabled: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct ToggleButtonGroupContext(pub(super) Signal<Option<GroupSettings>>);

impl ToggleButtonGroupContext {
    pub(crate) fn empty() -> Self {
        Self(Signal::new(None))
    }
}

/// Groups controlled toggle buttons with a shared selection and size default.
#[component]
pub fn ToggleButtonGroup(
    /// Accessible group name.
    label: String,
    /// Selection and corresponding change callback.
    selection: ToggleButtonGroupSelection,
    /// Toggle buttons and their wrappers.
    children: Element,
    /// Horizontal groups wrap; vertical groups stack.
    #[props(default)]
    orientation: ButtonGroupOrientation,
    /// Default member size; an explicit child size wins.
    #[props(default)]
    size: Option<ButtonSize>,
    /// Disables all member toggles.
    #[props(default)]
    is_disabled: bool,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline root styles.
    #[props(default)]
    style: Option<String>,
    /// Native root attributes. The component owns role and name.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let settings = Some(GroupSettings {
        selection,
        size,
        is_disabled,
    });
    let ToggleButtonGroupContext(mut context) =
        use_context_provider(|| ToggleButtonGroupContext(Signal::new(settings.clone())));
    if *context.peek() != settings {
        context.set(settings);
    }
    let direction = match orientation {
        ButtonGroupOrientation::Horizontal => "horizontal",
        ButtonGroupOrientation::Vertical => "vertical",
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "role" | "aria-label" | "aria-labelledby" | "data-orientation"
            )
        })
        .collect();
    rsx! {
        div { class: "astryx-toggle-button-group {class}", style, role: "group", "aria-label": label,
            "data-orientation": direction, ..attributes, {children}
        }
    }
}
