use dioxus::prelude::*;

/// Density inherited by list rows.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ListDensity {
    Compact,
    #[default]
    Balanced,
    Spacious,
}

/// Relationship of the list to the enclosing container's insets.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ListEdge {
    #[default]
    Padded,
    Flush,
    Outset,
    Bleed,
}

/// Marker style inherited by list rows. Decimal uses an ordered list.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ListMarkerStyle {
    #[default]
    None,
    Disc,
    Decimal,
    Circle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct ListSettings {
    pub(super) density: ListDensity,
    pub(super) edge: ListEdge,
    pub(super) marker_style: ListMarkerStyle,
    pub(super) has_dividers: bool,
}

#[derive(Clone, Copy)]
pub(super) struct ListContext(pub(super) Signal<ListSettings>);
