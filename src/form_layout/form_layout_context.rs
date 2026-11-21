//! Defaults published by FormLayout, scoped to the nearest layout and layer.

use dioxus::prelude::Signal;

/// Spatial arrangement of a FormLayout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormLayoutDirection {
    /// Stack fields vertically.
    #[default]
    Vertical,
    /// Give each field one equal grid track in logical inline order.
    Horizontal,
    /// Align labels beside controls, stacking at viewport widths of 480px or less.
    HorizontalLabels,
}

impl FormLayoutDirection {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::HorizontalLabels => "horizontal-labels",
        }
    }
}

/// Form-wide default for resolving field optionality.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormOptionality {
    /// Fields are optional unless explicitly required.
    Optional,
    /// Fields are required unless explicitly optional or exempt.
    Required,
}

/// Default time at which a compatible input reveals an entry error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryErrorBehavior {
    /// Use the field's normal entry-completion behavior.
    Complete,
    /// Defer presentation until blur or Enter.
    BlurOrEnter,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FormLayoutSettings {
    pub(crate) direction: FormLayoutDirection,
    pub(crate) default_optionality: Option<FormOptionality>,
    pub(crate) default_entry_error_behavior: Option<EntryErrorBehavior>,
}

pub(crate) type FormLayoutContext = Signal<FormLayoutSettings>;
