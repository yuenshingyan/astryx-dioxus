// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

/// Direction of a connected group.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    /// Upstream orientation selector.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// State owned by the nearest connected ButtonGroup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ButtonGroupContextValue {
    pub orientation: ButtonGroupOrientation,
    pub is_disabled: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct ButtonGroupContext(pub Signal<Option<ButtonGroupContextValue>>);

// Preserve actual group constraints when legacy slot adapters override their
// own synthetic group state. Core consumers only read ButtonGroupContext.
#[derive(Clone, Copy)]
pub(crate) struct ConnectedButtonGroupContext(pub Signal<ButtonGroupContextValue>);

pub(crate) fn use_button_group() -> Option<ButtonGroupContextValue> {
    try_consume_context::<ButtonGroupContext>().and_then(|context| (context.0)())
}
