// Adapted from Astryx ClickableCard/ClickableCard.tsx. See THIRD_PARTY_NOTICES.md.
use std::cell::Cell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, throw_val};
#[cfg(target_arch = "wasm32")]
use web_sys::{FocusOptions, HtmlElement, MouseEvent as DomMouseEvent};

use crate::card::{Card, CardVariant};
#[cfg(target_arch = "wasm32")]
use crate::hooks::use_clickable_container::native::{
    allows_activation, delegate_action, navigate, with_current_target,
};
use crate::hooks::use_clickable_container::{NativeOwner, use_native_owner};
use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_press_feedback};
use crate::link::{LinkRenderProps, use_link_component};
use crate::utils::disabled_interaction::{cancel_disabled_click, cancel_disabled_key};
use crate::utils::safe_url::is_safe_url;
use crate::utils::types::{Elevation, SizeValue, SpacingStep};

/// Invalid accessible name at the public component boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClickableCardError {
    /// Whitespace cannot name the native action.
    EmptyLabel,
}
impl Display for ClickableCardError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ClickableCard label must contain non-whitespace text")
    }
}
impl Error for ClickableCardError {}

/// Card surface with one hidden native action and independent nested controls.
///
/// `href` selects navigation over action mode. Disabled controls remain focusable
/// but inert. A missing destination and callback retains the released focusable
/// no-op composition; the upstream OQ1 policy remains unresolved.
#[component]
pub fn ClickableCard(
    /// Accessible name for the hidden native link or button.
    label: String,
    /// Surface action; hidden button activation invokes it exactly once too.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Optional destination. Every sink uses the shared URL acceptance rule.
    #[props(default)]
    href: Option<String>,
    /// Optional native link target.
    #[props(default)]
    target: Option<String>,
    /// Focusable, announced, inert disabled state.
    #[props(default)]
    is_disabled: bool,
    /// Card's theme-derived padding when omitted.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Card surface variant, including validated custom theme names.
    #[props(default)]
    variant: CardVariant,
    /// Resting Card shadow.
    #[props(default)]
    elevation: Elevation,
    /// Preferred Card width.
    #[props(default)]
    width: Option<SizeValue>,
    /// Explicit Card height.
    #[props(default)]
    height: Option<SizeValue>,
    /// Maximum Card width.
    #[props(default)]
    max_width: Option<SizeValue>,
    /// Additional root class.
    #[props(default)]
    class: String,
    /// Final caller root styles.
    #[props(default)]
    style: String,
    /// Root ref callback, after internal resource installation.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Runs after internal mouseup handling, and is omitted while disabled.
    #[props(default)]
    onmouseup: Option<EventHandler<MouseEvent>>,
    /// Caller pointerdown, retained while disabled.
    #[props(default)]
    onpointerdown: Option<EventHandler<PointerEvent>>,
    /// Caller content keeps its own semantics and interaction.
    #[props(default = rsx! {})]
    children: Element,
    /// Native attributes and non-reserved events. `onclick`, `onmouseup`,
    /// `onpointerdown`, and `onmounted` are reserved; use the named typed props.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| format!("astryx-clickable-{}", current_scope_id().0));
    let (root_owner, root_ref) = use_native_owner();
    let (action_owner, action_ref) = use_native_owner();
    let is_delegating = use_hook(|| Rc::new(Cell::new(false)));
    let restore_focus = use_focus_restore(
        is_disabled,
        href.is_some(),
        action_owner.clone(),
        action_ref,
    );
    let press = use_press_feedback(PressFeedbackConfig {
        id: id.clone(),
        label: label.clone(),
        tooltip: None,
        is_icon_only: false,
        is_loading: false,
        is_disabled,
    });
    let renderer = use_link_component(None);
    if label.trim().is_empty() {
        return Err(ClickableCardError::EmptyLabel.into());
    }
    let callbacks = CardCallbacks {
        root_owner: root_owner.clone(),
        action_owner: action_owner.clone(),
        is_delegating: is_delegating.clone(),
        href: href.clone(),
        target: target.clone(),
        onclick,
        onmouseup,
    };
    let remaining_attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "data-disabled"
                    | "data-astryx-feedback-id"
                    | "data-astryx-container-press"
                    | "data-pressable-container"
                    | "onclick"
                    | "onmouseup"
                    | "onpointerdown"
                    | "onmounted"
            )
        })
        .collect();
    // Retain inert declarative listener slots while disabled. Removing one can
    // make the web renderer clear another retained callback's event ownership.
    let mut attributes = vec![
        Attribute::new("data-astryx-feedback-id", id, None, false),
        Attribute::new("data-pressable-container", "true", None, false),
        Attribute::new(
            "data-astryx-container-press",
            if is_disabled { "false" } else { "true" },
            None,
            false,
        ),
        Attribute::new("data-disabled", is_disabled.then_some("true"), None, false),
    ];
    let click_callbacks = callbacks.clone();
    attributes.push(native_attribute("onclick", move |event| {
        if !is_disabled {
            click_callbacks.click(event);
        }
    }));
    attributes.push(native_attribute("onmouseup", move |event| {
        if !is_disabled {
            callbacks.mouseup(event);
        }
    }));
    attributes.push(native_attribute("onpointerdown", move |event| {
        if let Some(handler) = onpointerdown {
            let pointer = event.map(|data| PointerData::from(data));
            let scope_pointer = pointer.clone();
            scope_pointer_callback(&root_owner, &scope_pointer, move || handler.call(pointer));
        }
    }));
    attributes.extend(remaining_attributes);
    let leaf_delegation = is_delegating.clone();
    let link_click = EventHandler::new(move |event: MouseEvent| {
        if leaf_delegation.get() {
            // Only an internal proxy stops at its own leaf. Ordinary child
            // callbacks retain normal bubbling and run before the card's root.
            event.stop_propagation();
        }
    });
    let is_link = href.is_some();
    let accepted_href = href.filter(|href| !is_disabled && is_safe_url(href));
    let link_target = (!is_disabled).then_some(target).flatten();
    let action_class = "astryx-clickable-card__action";
    let action = if is_link {
        if let Some(renderer) = renderer.filter(|_| !is_disabled && accepted_href.is_some()) {
            renderer.call(LinkRenderProps {
                href: accepted_href,
                target: link_target,
                rel: None,
                attributes: vec![
                    Attribute::new("class", action_class, None, false),
                    Attribute::new("data-astryx-container-action", "", None, false),
                    Attribute::new("aria-label", label.clone(), None, false),
                    Attribute::new("tabindex", "0", None, false),
                ],
                onclick: link_click,
                onkeydown: EventHandler::new(|_| {}),
                onmounted: restore_focus,
                children: rsx! {},
            })
        } else {
            rsx! {a {
                class: action_class, "data-astryx-container-action": "", "aria-label": label,
                href: accepted_href, target: link_target, role: "link", tabindex: 0,
                "aria-disabled": is_disabled.then_some("true"), onmounted: restore_focus,
                onclick: move |event:MouseEvent| { if is_disabled { cancel_disabled_click(&event); } else {link_click.call(event);} },
                onkeydown: move |event:KeyboardEvent| {if is_disabled {cancel_disabled_key(&event);}},
            }}
        }
    } else {
        let action_click = native_attribute("onclick", move |event| {
            if is_disabled {
                cancel_disabled_click(&event);
                return;
            }
            if is_delegating.get() {
                event.stop_propagation();
            }
            invoke_mouse_callback(
                &action_owner,
                event.map(|data| MouseData::from(data)),
                onclick,
            );
        });
        rsx! {button {
            class: action_class, "data-astryx-container-action": "", r#type: "button", "aria-label": label,
            "aria-disabled": is_disabled.then_some("true"), onmounted: restore_focus,
            onkeydown: move |event:KeyboardEvent| {if is_disabled {cancel_disabled_key(&event);}},
            ..vec![action_click],
        }}
    };
    rsx! {Card {
        width,height,max_width,padding,variant,elevation,
        class:format!("astryx-clickable-card {class}"),style,attributes,
        onmounted:move |event:MountedEvent| {
            root_ref.call(event.clone());press.call(event.clone());
            if let Some(handler)=onmounted {handler.call(event);}
        },
        {action}{children}
    }}
}

fn native_attribute(
    name: &'static str,
    callback: impl FnMut(Event<PlatformEventData>) + 'static,
) -> Attribute {
    Attribute::new(name, AttributeValue::listener(callback), None, false)
}
fn invoke_mouse_callback(
    owner: &NativeOwner,
    event: MouseEvent,
    handler: Option<EventHandler<MouseEvent>>,
) -> bool {
    let Some(handler) = handler else {
        return true;
    };
    let scope_event = event.clone();
    scope_mouse_callback(owner, &scope_event, move || handler.call(event))
}

#[derive(Clone)]
struct CardCallbacks {
    root_owner: NativeOwner,
    action_owner: NativeOwner,
    is_delegating: Rc<Cell<bool>>,
    href: Option<String>,
    target: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
}
impl CardCallbacks {
    fn click(&self, event: Event<PlatformEventData>) {
        let mouse = event.clone().map(|data| MouseData::from(data));
        if !allows_surface(&self.root_owner, &mouse) {
            return;
        }
        if !invoke_mouse_callback(&self.root_owner, mouse.clone(), self.onclick)
            || !event.default_action_enabled()
        {
            return;
        }
        activate_surface(self, &mouse);
    }
    fn mouseup(&self, event: Event<PlatformEventData>) {
        let mouse = event.clone().map(|data| MouseData::from(data));
        middle_navigation(self, &mouse);
        invoke_mouse_callback(&self.root_owner, mouse, self.onmouseup);
    }
}

#[cfg(target_arch = "wasm32")]
fn allows_surface(owner: &NativeOwner, event: &MouseEvent) -> bool {
    owner
        .element()
        .zip(event.data().downcast::<DomMouseEvent>())
        .is_some_and(|(root, native)| {
            native.button() == 0 && allows_activation(native, &root, false)
        })
}
#[cfg(not(target_arch = "wasm32"))]
fn allows_surface(_owner: &NativeOwner, _event: &MouseEvent) -> bool {
    true
}

#[cfg(target_arch = "wasm32")]
fn scope_mouse_callback(
    owner: &NativeOwner,
    event: &MouseEvent,
    callback: impl FnOnce() + 'static,
) -> bool {
    let result = {
        let Some(owner) = owner.element() else {
            return false;
        };
        let data = event.data();
        let Some(native) = data.downcast::<DomMouseEvent>() else {
            return false;
        };
        with_current_target(native.as_ref(), &owner, callback)
    };
    match result {
        Ok(()) => true,
        Err(err) => throw_val(err),
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn scope_mouse_callback(
    _owner: &NativeOwner,
    _event: &MouseEvent,
    callback: impl FnOnce() + 'static,
) -> bool {
    callback();
    true
}

#[cfg(target_arch = "wasm32")]
fn scope_pointer_callback(
    owner: &NativeOwner,
    event: &PointerEvent,
    callback: impl FnOnce() + 'static,
) {
    let result = {
        let Some(owner) = owner.element() else {
            return;
        };
        let data = event.data();
        let Some(native) = data.downcast::<web_sys::PointerEvent>() else {
            return;
        };
        with_current_target(native.as_ref(), &owner, callback)
    };
    if let Err(err) = result {
        throw_val(err);
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn scope_pointer_callback(
    _owner: &NativeOwner,
    _event: &PointerEvent,
    callback: impl FnOnce() + 'static,
) {
    callback();
}

#[cfg(target_arch = "wasm32")]
struct DelegationScope {
    state: Rc<Cell<bool>>,
    previous: bool,
}
#[cfg(target_arch = "wasm32")]
impl DelegationScope {
    fn new(state: Rc<Cell<bool>>) -> Self {
        let previous = state.replace(true);
        Self { state, previous }
    }
}
#[cfg(target_arch = "wasm32")]
impl Drop for DelegationScope {
    fn drop(&mut self) {
        self.state.set(self.previous);
    }
}

#[cfg(target_arch = "wasm32")]
fn activate_surface(callbacks: &CardCallbacks, event: &MouseEvent) {
    let data = event.data();
    let Some(native) = data.downcast::<DomMouseEvent>() else {
        return;
    };
    let action = callbacks
        .action_owner
        .element()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok());
    let _scope = DelegationScope::new(callbacks.is_delegating.clone());
    if let Some(href) = &callbacks.href {
        navigate(native, href, callbacks.target.as_deref(), action.as_ref());
    } else if callbacks.onclick.is_none()
        && let Some(action) = action
    {
        delegate_action(native, &action);
        event.stop_propagation();
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn activate_surface(callbacks: &CardCallbacks, _event: &MouseEvent) {
    let _ = (
        &callbacks.href,
        &callbacks.target,
        &callbacks.action_owner,
        &callbacks.is_delegating,
    );
}

#[cfg(target_arch = "wasm32")]
fn middle_navigation(callbacks: &CardCallbacks, event: &MouseEvent) {
    let data = event.data();
    if let Some(native) = data.downcast::<DomMouseEvent>()
        && native.button() == 1
        && let Some(root) = callbacks.root_owner.element()
        && allows_activation(native, &root, false)
        && let Some(href) = &callbacks.href
    {
        navigate(native, href, callbacks.target.as_deref(), None);
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn middle_navigation(_callbacks: &CardCallbacks, _event: &MouseEvent) {}

#[cfg(target_arch = "wasm32")]
fn use_focus_restore(
    is_disabled: bool,
    is_link: bool,
    owner: NativeOwner,
    mounted: EventHandler<MountedEvent>,
) -> EventHandler<MountedEvent> {
    let previous_disabled = use_hook(|| Rc::new(Cell::new(is_disabled)));
    let should_restore = use_hook(|| Rc::new(Cell::new(false)));
    if previous_disabled.replace(is_disabled) != is_disabled && is_link {
        should_restore.set(owner.element().is_some_and(|element| {
            element
                .owner_document()
                .is_some_and(|document| document.active_element().as_ref() == Some(&element))
        }));
    }
    EventHandler::new(move |event| {
        mounted.call(event);
        if !should_restore.replace(false) {
            return;
        }
        if let Some(element) = owner
            .element()
            .and_then(|element| element.dyn_into::<HtmlElement>().ok())
            && element.owner_document().is_some_and(|document| {
                document
                    .active_element()
                    .is_none_or(|active| active.tag_name() == "BODY")
            })
        {
            let options = FocusOptions::new();
            options.set_prevent_scroll(true);
            let _ = element.focus_with_options(&options);
        }
    })
}
#[cfg(not(target_arch = "wasm32"))]
fn use_focus_restore(
    _is_disabled: bool,
    _is_link: bool,
    _owner: NativeOwner,
    mounted: EventHandler<MountedEvent>,
) -> EventHandler<MountedEvent> {
    mounted
}
