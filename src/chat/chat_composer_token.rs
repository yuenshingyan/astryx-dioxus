use dioxus::prelude::*;

use crate::BadgeVariant;

/// A serialized chat composer token and its inline presentation.
#[derive(Clone, PartialEq)]
pub enum ChatComposerToken {
    /// A token displayed as a badge.
    Badge {
        value: String,
        label: String,
        variant: BadgeVariant,
        icon: Option<Element>,
    },
    /// A token with caller-owned rendering.
    Custom {
        value: String,
        render: Callback<(), Element>,
    },
}

impl ChatComposerToken {
    /// Literal serialized value used to match message text.
    pub fn value(&self) -> &str {
        match self {
            Self::Badge { value, .. } | Self::Custom { value, .. } => value,
        }
    }
}
