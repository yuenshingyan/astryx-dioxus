// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

use crate::compatibility::publish_group_environment;
use crate::size_context::{ElementSize, SizeProvider, use_size};

use super::button_group_context::{
    ButtonGroupContext, ButtonGroupContextValue, ButtonGroupOrientation,
    ConnectedButtonGroupContext,
};

/// Resting shadow of the connected group as a single surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonGroupElevation {
    #[default]
    None,
    Low,
    Medium,
    High,
}

impl ButtonGroupElevation {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "med",
            Self::High => "high",
        }
    }
}

/// Connected button surface and environment provider.
///
/// `label` must name the group (whitespace is trimmed). Browser integration
/// owns roving focus and touch tracking through `data-astryx-button-group`;
/// this component does not install listeners or alter children's tab indices.
#[component]
pub fn ButtonGroup(
    label: String,
    children: Element,
    #[props(default)] size: Option<ElementSize>,
    #[props(default)] orientation: ButtonGroupOrientation,
    #[props(default)] is_disabled: bool,
    #[props(default)] elevation: ButtonGroupElevation,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let resolved_size = use_size(size);
    let value = ButtonGroupContextValue {
        is_disabled,
        orientation,
    };
    let mut context = use_context_provider(|| ButtonGroupContext(Signal::new(Some(value))));
    if *context.0.peek() != Some(value) {
        context.0.set(Some(value));
    }
    let mut connected = use_context_provider(|| ConnectedButtonGroupContext(Signal::new(value)));
    if *connected.0.peek() != value {
        connected.0.set(value);
    }
    publish_group_environment(resolved_size, is_disabled, orientation);
    let label = label.trim();
    rsx! {
        SizeProvider { size: resolved_size,
            div {
                class: "astryx-button-group astryx-button-group--{orientation.as_str()} {class}",
                style,
                role: "group",
                "aria-label": (!label.is_empty()).then_some(label),
                "aria-disabled": is_disabled.then_some("true"),
                "data-astryx-button-group": "",
                "data-size": resolved_size.as_str(),
                "data-orientation": orientation.as_str(),
                "data-elevation": elevation.as_str(),
                "data-disabled": is_disabled.then_some("true"),
                {children}
            }
        }
    }
}
