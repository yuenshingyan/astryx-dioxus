//! Compatibility for the provisional combined environment API.
//!
//! This adapter is not an Astryx primitive. Core components use the separate
//! upstream-owned ButtonGroup, InputGroup, and Size contexts instead. Retain
//! this API for existing callers; remove it only with breaking-change approval.

use dioxus::prelude::*;

use crate::button_group::ButtonGroupOrientation;
use crate::button_group::button_group_context::{
    ButtonGroupContext, ButtonGroupContextValue, ConnectedButtonGroupContext, use_button_group,
};
use crate::input_group::input_group_internal_context::{
    InputGroupInternalContext, InputGroupInternalState,
};
use crate::size_context::{ElementSize, SizeContext, SizeProvider};

/// Legacy slot settings. Prefer SizeProvider and upstream-owned group APIs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ButtonEnvironment {
    pub size: Option<ElementSize>,
    pub is_disabled: bool,
    pub is_grouped: bool,
    pub orientation: ButtonGroupOrientation,
}

/// Legacy context retained for source compatibility, not used by core components.
#[derive(Clone, Copy, PartialEq)]
pub struct ButtonEnvironmentContext(pub Signal<ButtonEnvironment>);

/// Reads legacy settings, including constraints from a real connected group.
pub fn use_button_environment() -> ButtonEnvironment {
    let environment = try_consume_context::<ButtonEnvironmentContext>()
        .map(|context| (context.0)())
        .unwrap_or_default();
    if let Some(group) = use_button_group() {
        let inherited_size = try_consume_context::<SizeContext>().map(|context| (context.0)());
        return ButtonEnvironment {
            size: environment.size.or(inherited_size),
            is_disabled: environment.is_disabled || group.is_disabled,
            is_grouped: true,
            orientation: group.orientation,
        };
    }
    environment
}

// Keep direct consumers of the old public context working under ButtonGroup.
// This snapshot is never used to resolve core component state.
pub(crate) fn publish_group_environment(
    size: ElementSize,
    is_disabled: bool,
    orientation: ButtonGroupOrientation,
) {
    let environment = ButtonEnvironment {
        size: Some(size),
        is_disabled,
        is_grouped: true,
        orientation,
    };
    let mut context = use_context_provider(|| ButtonEnvironmentContext(Signal::new(environment)));
    if *context.0.peek() != environment {
        context.0.set(environment);
    }
}

/// Compatibility adapter from provisional settings to independent contexts.
///
/// A nested adapter replaces another adapter's flags, but cannot clear a real
/// ButtonGroup's constraints. It renders no additional DOM element.
#[component]
pub fn ButtonEnvironmentProvider(environment: ButtonEnvironment, children: Element) -> Element {
    let inherited_size = try_consume_context::<SizeContext>().map(|context| (context.0)());
    let size = environment.size.or(inherited_size).unwrap_or_default();
    let connected =
        try_consume_context::<ConnectedButtonGroupContext>().and_then(|context| (context.0)());
    let group = connected.or_else(|| {
        environment.is_grouped.then_some(ButtonGroupContextValue {
            orientation: environment.orientation,
            is_disabled: environment.is_disabled,
        })
    });
    let input = InputGroupInternalState {
        is_disabled: environment.is_disabled,
    };
    let mut legacy = use_context_provider(|| ButtonEnvironmentContext(Signal::new(environment)));
    if *legacy.0.peek() != environment {
        legacy.0.set(environment);
    }
    let mut group_context = use_context_provider(|| ButtonGroupContext(Signal::new(group)));
    if *group_context.0.peek() != group {
        group_context.0.set(group);
    }
    let mut input_context = use_context_provider(|| InputGroupInternalContext(Signal::new(input)));
    if *input_context.0.peek() != input {
        input_context.0.set(input);
    }
    rsx! { SizeProvider { size, {children} } }
}
