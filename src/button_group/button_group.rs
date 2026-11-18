// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::compatibility::publish_group_environment;
use crate::size_context::{ElementSize, SizeProvider, use_size};

use super::button_group_context::{
    ButtonGroupContext, ButtonGroupContextValue, ButtonGroupOrientation,
    ConnectedButtonGroupContext,
};

#[derive(Debug)]
enum ButtonGroupError {
    MissingLabel,
}

impl Display for ButtonGroupError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::MissingLabel => {
                formatter.write_str("ButtonGroup requires a non-empty accessible label")
            }
        }
    }
}

impl Error for ButtonGroupError {}

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

/// A connected, labelled surface for Button and IconButton members.
///
/// `label` must be non-empty (whitespace is trimmed); invalid names return a
/// recoverable render error. `onfocus` reports member focus via bubbling focusin.
/// Browser integration
/// owns roving focus and touch tracking through `data-astryx-button-group`;
/// the scope-owned controller maintains one Tab stop, logical connected corners,
/// and isolation from member-owned layers. Root event callbacks run before its
/// keyboard handling; prevent_default() cancels the group's key behavior.
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
    #[props(default)] onkeydown: EventHandler<KeyboardEvent>,
    #[props(default)] onfocus: EventHandler<FocusEvent>,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let controller_id = use_hook(|| format!("astryx-button-group-{}", current_scope_id().0));
    let mut browser_controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(controller) = *browser_controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    let mounted_id = controller_id.clone();
    let handle_mounted = move |event: MountedEvent| {
        if let Some(previous) = *browser_controller.peek() {
            let _ = previous.send("dispose");
        }
        let controller = document::eval(include_str!("../../assets/button-group-controller.js"));
        let _ = controller.send(mounted_id.clone());
        browser_controller.set(Some(controller));
        onmounted.call(event);
    };
    let attributes = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class"
                    | "style"
                    | "role"
                    | "aria-label"
                    | "aria-disabled"
                    | "aria-orientation"
                    | "data-size"
                    | "data-orientation"
                    | "data-elevation"
                    | "data-disabled"
                    | "data-astryx-button-group"
                    | "data-astryx-button-group-id"
                    | "data-astryx-touch-action"
                    | "onkeydown"
                    | "onfocus"
                    | "onfocusin"
                    | "onmounted"
            )
        })
        .collect::<Vec<_>>();
    let resolved_size = use_size(size);
    let value = ButtonGroupContextValue {
        is_disabled,
        orientation,
    };
    let mut context = use_context_provider(|| ButtonGroupContext(Signal::new(Some(value))));
    if *context.0.peek() != Some(value) {
        context.0.set(Some(value));
    }
    let mut connected =
        use_context_provider(|| ConnectedButtonGroupContext(Signal::new(Some(value))));
    if *connected.0.peek() != Some(value) {
        connected.0.set(Some(value));
    }
    publish_group_environment(resolved_size, is_disabled, orientation);
    let label = label.trim();
    if label.is_empty() {
        return Err(ButtonGroupError::MissingLabel.into());
    }
    rsx! {
        SizeProvider { size: resolved_size,
            div {
                class: "astryx-button-group astryx-button-group--{orientation.as_str()} {class}",
                style,
                onmounted: handle_mounted,
                onkeydown,
                onfocusin: onfocus,
                role: "group",
                "aria-label": (!label.is_empty()).then_some(label),
                "aria-disabled": is_disabled.then_some("true"),
                "data-astryx-button-group": "",
                "data-astryx-button-group-id": controller_id,
                "data-size": resolved_size.as_str(),
                "data-orientation": orientation.as_str(),
                "data-elevation": elevation.as_str(),
                "data-disabled": is_disabled.then_some("true"),
                ..attributes,
                {children}
            }
        }
    }
}
