// Adapted from Meta Astryx 476a3153417dbf75c565f7df05b0784e94ddd59e.
use std::error::Error;
use std::fmt::{Display, Formatter};
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

use dioxus::core::current_scope_id;
use dioxus::events::{
    onclick as click_attribute, onmouseup as mouseup_attribute,
    onpointerdown as pointerdown_attribute,
};
use dioxus::prelude::*;

use super::selectable_card_input::{
    SelectableCardInputContext, SelectableCardInputType, requested_selection,
};
use crate::Card;
use crate::card::CardVariant;
use crate::hooks::use_clickable_container::use_native_owner;
use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_press_feedback};
use crate::utils::disabled_interaction::cancel_disabled_key;
use crate::utils::types::{Elevation, SizeValue, SpacingStep};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{
    Element as DomElement, Event as DomEvent, HtmlInputElement, MouseEvent as DomMouseEvent,
};

#[cfg(target_arch = "wasm32")]
use super::native::{
    DeferredDefault, call_mouse_owner, call_pointer_owner, owns_portaled_layer, restore_checked,
};
#[cfg(target_arch = "wasm32")]
use crate::hooks::use_clickable_container::native::{
    allows_activation, apply_cancellation, mouse_event, use_native_document_listener,
    use_native_listener,
};
#[cfg(target_arch = "wasm32")]
use crate::private_attributes::layer_root::closest_layer_root;

/// A required accessible label was empty or whitespace-only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectableCardError {
    /// Supply meaningful accessible text through `label`.
    EmptyLabel,
}
impl Display for SelectableCardError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SelectableCard label must contain non-whitespace text")
    }
}
impl Error for SelectableCardError {}

/// A controlled checkbox option painted on a Card surface.
///
/// The first child is a named, visually hidden native checkbox. Space acts
/// natively; Enter requests a change without synthesizing a click. Nested
/// controls keep their actions. The root itself has no role or tab stop.
/// `onchange` requests selection; the caller must update `is_selected` to accept it.
/// Single-choice application examples still expose checkboxes, not a radio group.
#[component]
pub fn SelectableCard(
    /// Accessible name for the native checkbox. Must contain non-whitespace text.
    label: String,
    /// Controlled selected state; never inferred from the DOM.
    is_selected: bool,
    /// Selection request. Application operations and state remain caller-owned.
    onchange: EventHandler<bool>,
    /// Disabled options remain focusable and announce aria-disabled.
    #[props(default)]
    is_disabled: bool,
    /// Card's theme-extensible surface variant.
    #[props(default)]
    variant: CardVariant,
    /// Resting shadow, composed with the two-pixel inset selection ring.
    #[props(default)]
    elevation: Elevation,
    /// Omission reads Card's theme padding.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Preferred Card width.
    #[props(default)]
    width: Option<SizeValue>,
    /// Explicit non-auto height enables Card scrolling.
    #[props(default)]
    height: Option<SizeValue>,
    /// Maximum Card width.
    #[props(default)]
    max_width: Option<SizeValue>,
    /// Additional root classes; Card styling is not duplicated here.
    #[props(default)]
    class: String,
    /// Caller root style, applied last.
    #[props(default)]
    style: String,
    /// Runs for enabled surface and nested clicks, including open-layer capture.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Runs after internal mouseup handling; omitted while disabled.
    #[props(default)]
    onmouseup: Option<EventHandler<MouseEvent>>,
    /// Caller pointerdown event, including while disabled.
    #[props(default)]
    onpointerdown: Option<EventHandler<PointerEvent>>,
    /// Optional native capture callback, after the released layer forwarding path.
    #[props(default)]
    onclick_capture: Option<EventHandler<MouseEvent>>,
    /// Ref callback for the Card root.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Arbitrary content; nested controls retain their own semantics.
    #[props(default = rsx!{})]
    children: Element,
    /// Native root attributes and non-reserved events. Use named props for
    /// onclick, onmouseup, onpointerdown, onmounted and onclick_capture;
    /// raw attribute builders for these component-owned names are ignored.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    if label.trim().is_empty() {
        return Err(SelectableCardError::EmptyLabel.into());
    }
    let input = try_use_context::<SelectableCardInputContext>()
        .and_then(|context| context.0.read().clone());
    let input_type = input
        .as_ref()
        .map_or(SelectableCardInputType::Checkbox, |input| input.input_type);
    let is_radio = input_type == SelectableCardInputType::Radio;
    let request_change = EventHandler::new(move |_| {
        if !is_disabled && let Some(is_now_selected) = requested_selection(is_radio, is_selected) {
            onchange.call(is_now_selected);
        }
    });
    let id = use_hook(|| format!("astryx-selectable-card-{}", current_scope_id().0));
    let press_mounted = use_press_feedback(PressFeedbackConfig {
        id: id.clone(),
        label: label.clone(),
        tooltip: None,
        is_icon_only: false,
        is_loading: false,
        is_disabled,
    });
    let (root_owner, owner_mounted) = use_native_owner();
    #[cfg(target_arch = "wasm32")]
    let capture_owner = root_owner.clone();
    #[cfg(target_arch = "wasm32")]
    let click_owner = root_owner.clone();
    #[cfg(target_arch = "wasm32")]
    let mouseup_owner = root_owner.clone();
    #[cfg(target_arch = "wasm32")]
    let pointer_owner = root_owner.clone();
    #[cfg(not(target_arch = "wasm32"))]
    let _ = root_owner;
    #[cfg(target_arch = "wasm32")]
    let native_mounted = {
        let selector = format!("[data-astryx-feedback-id=\"{id}\"]");
        let mut deferred = use_signal(|| None::<DeferredDefault>);
        use_drop(move || {
            deferred.write().take();
        });
        let capture = use_callback(move |event: DomEvent| {
            // Also removes a pending deferral if an earlier layer stopped the
            // original event before it could reach document.
            deferred.write().take();
            let Some(mouse) = event.dyn_ref::<DomMouseEvent>() else {
                return;
            };
            let Some(root) = capture_owner.element() else {
                return;
            };
            let layer = event
                .target()
                .and_then(|target| target.dyn_into::<DomElement>().ok())
                .and_then(|target| closest_layer_root(&target));
            if !is_disabled
                && layer.is_some_and(|layer| !layer.contains(Some(&root)))
                && let Some(onclick) = onclick
            {
                let wrapped = mouse_event(mouse);
                call_mouse_owner(onclick, wrapped.clone(), &root);
                // Do not apply stopPropagation or expose preventDefault to
                // layer controls. This is the released, not next-minor path.
                if !wrapped.default_action_enabled() {
                    deferred.set(Some(DeferredDefault::new(mouse, &root)));
                }
            }
            if let Some(caller) = onclick_capture {
                let wrapped = mouse_event(mouse);
                call_mouse_owner(caller, wrapped.clone(), &root);
                apply_cancellation(&wrapped, &event);
            }
        });
        let capture_mounted = use_native_listener(selector, "click", true, capture);
        let portal_owner = root_owner.clone();
        let portal_mounted = use_native_document_listener(
            "click",
            true,
            Rc::new(move |event: DomEvent| {
                if event.cancel_bubble() {
                    return;
                }
                let Some(root) = portal_owner.element() else {
                    return;
                };
                let Some(target) = event
                    .target()
                    .and_then(|target| target.dyn_into::<DomElement>().ok())
                else {
                    return;
                };
                // DOM-contained events already take the ordinary native capture path.
                // Only Layer-owned logical portal ancestry bridges outside that path.
                if root.contains(Some(&target)) {
                    return;
                }
                if let Some(layer) = closest_layer_root(&target)
                    && owns_portaled_layer(&root, &layer)
                {
                    capture.call(event);
                }
            }),
        );
        EventHandler::new(move |event: MountedEvent| {
            capture_mounted.call(event.clone());
            portal_mounted.call(event);
        })
    };
    #[cfg(not(target_arch = "wasm32"))]
    let _ = onclick_capture;
    let handle_click = EventHandler::new(move |event: MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        if let (Some(native), Some(root)) = (
            event.data().downcast::<DomMouseEvent>(),
            click_owner.element(),
        ) {
            if is_disabled {
                if native
                    .target()
                    .and_then(|target| target.dyn_into::<DomElement>().ok())
                    .is_some_and(|target| root.first_element_child().as_ref() == Some(&target))
                {
                    event.prevent_default();
                }
                return;
            }
            if native
                .target()
                .and_then(|target| target.dyn_into::<DomElement>().ok())
                .and_then(|target| closest_layer_root(&target))
                .is_some_and(|layer| !layer.contains(Some(&root)))
            {
                return;
            }
            if allows_activation(native, &root, false) {
                request_change.call(());
            }
            if let Some(caller) = onclick {
                call_mouse_owner(caller, event, &root);
            }
            return;
        }
        if !is_disabled && let Some(caller) = onclick {
            caller.call(event);
        }
    });
    let mut attributes = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class"
                    | "style"
                    | "data-selected"
                    | "data-disabled"
                    | "data-astryx-feedback-id"
                    | "data-astryx-container-press"
                    | "data-pressable-container"
                    | "onclick"
                    | "onmouseup"
                    | "onpointerdown"
                    | "onmounted"
                    | "onclick_capture"
            )
        })
        .collect::<Vec<_>>();
    attributes.push(click_attribute(move |event| handle_click.call(event)));
    // Keep the renderer's event slots stable across disabled updates. Removing
    // one listener can otherwise remove its shared delegated-node identifier,
    // starving unrelated caller handlers that the disabled contract preserves.
    attributes.push(mouseup_attribute(move |event: MouseEvent| {
        if !is_disabled && let Some(caller) = onmouseup {
            #[cfg(target_arch = "wasm32")]
            if let Some(root) = mouseup_owner.element() {
                call_mouse_owner(caller, event, &root);
                return;
            }
            caller.call(event);
        }
    }));
    attributes.push(pointerdown_attribute(move |event: PointerEvent| {
        if let Some(caller) = onpointerdown {
            #[cfg(target_arch = "wasm32")]
            if let Some(root) = pointer_owner.element() {
                call_pointer_owner(caller, event, &root);
                return;
            }
            caller.call(event);
        }
    }));
    rsx! {
        Card {
            variant, elevation, padding, width, height, max_width,
            class: "astryx-selectable-card {class}", style,
            "data-selected": if is_selected { "true" } else { "false" },
            "data-disabled": if is_disabled { "true" } else { "false" },
            "data-pressable-container": "true",
            "data-astryx-container-press": if is_disabled { "false" } else { "true" },
            "data-astryx-feedback-id": id,
            onmounted: move |event: MountedEvent| {
                owner_mounted.call(event.clone());
                press_mounted.call(event.clone());
                #[cfg(target_arch = "wasm32")]
                native_mounted.call(event.clone());
                if let Some(onmounted) = onmounted { onmounted.call(event); }
            },
            attributes,
            input {
                class: "astryx-selectable-card__input",
                "data-astryx-container-action": "",
                r#type: if is_radio { "radio" } else { "checkbox" },
                name: input.as_ref().map(|input| input.name.clone()),
                value: input.as_ref().map(|input| input.value.clone()),
                form: input.as_ref().and_then(|_| (is_radio || is_disabled).then_some("")),
                checked: is_selected,
                aria_label: label,
                aria_disabled: is_disabled.then_some("true"),
                onchange: move |event: FormEvent| {
                    request_change.call(());
                    // A rejected request need not cause a render. Repair the
                    // actual checked property even in that controlled case.
                    #[cfg(target_arch = "wasm32")]
                    if let Some(input) = event.data().downcast::<DomEvent>().and_then(|event| event.target()).and_then(|target| target.dyn_into::<HtmlInputElement>().ok()) {
                        restore_checked(&input, is_selected);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    let _ = event;
                },
                onkeydown: move |event: KeyboardEvent| {
                    let is_ime = event.is_composing();
                    #[cfg(target_arch = "wasm32")]
                    let is_ime = is_ime || event.data().downcast::<web_sys::KeyboardEvent>().is_some_and(|event| event.key_code() == 229);
                    if is_ime { return; }
                    if is_disabled {
                        cancel_disabled_key(&event);
                    } else if event.key() == Key::Enter {
                        event.prevent_default(); request_change.call(());
                    }
                },
            }
            {children}
        }
    }
}
