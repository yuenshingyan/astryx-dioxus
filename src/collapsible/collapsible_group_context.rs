use dioxus::prelude::*;

/// Block padding for disclosure rows.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CollapsibleGroupDensity {
    Compact,
    Balanced,
    Spacious,
}

impl CollapsibleGroupDensity {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Balanced => "balanced",
            Self::Spacious => "spacious",
        }
    }
}

#[derive(Clone, PartialEq)]
pub(super) struct GroupSelection {
    pub(super) open_values: Vec<String>,
    pub(super) toggle: EventHandler<String>,
}

#[derive(Clone, Copy)]
pub(super) struct CollapsibleGroupContext(pub(super) Signal<GroupSelection>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct GroupPresentation {
    pub(super) has_dividers: bool,
    pub(super) density: Option<CollapsibleGroupDensity>,
}

#[derive(Clone, Copy)]
pub(super) struct CollapsibleGroupPresentationContext(pub(super) Signal<Option<GroupPresentation>>);
