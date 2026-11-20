// Adapted from Astryx hooks/useClickableContainer.ts. See THIRD_PARTY_NOTICES.md.

/// Nested controls own their activation, including marked nested containers.
pub const INTERACTIVE_SELECTORS: &str = "button,a,input,select,textarea,[role=button],[role=link],[role=checkbox],[role=radio],[role=switch],[role=tab],[role=menuitem],[role=option],[role=combobox],[role=listbox],[role=slider],[role=spinbutton],[data-pressable-container]";

/// Native event boundary. No renderer or DOM is required for host rendering.
#[cfg(target_arch = "wasm32")]
pub mod native {
    use std::any::Any;
    use std::rc::Rc;

    use dioxus::html::geometry::{ClientPoint, ElementPoint, PagePoint, ScreenPoint};
    use dioxus::html::input_data::{MouseButton, MouseButtonSet, decode_mouse_button_set};
    use dioxus::html::{HasMouseData, Modifiers};
    use dioxus::prelude::*;
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{Element, Event as DomEvent, EventTarget, MouseEvent as DomMouseEvent};

    use super::INTERACTIVE_SELECTORS;

    /// The original event remains downcastable, including during capture.
    pub struct NativeMouseData(pub DomMouseEvent);

    impl HasMouseData for NativeMouseData {
        fn as_any(&self) -> &dyn Any {
            &self.0
        }
    }
    impl InteractionLocation for NativeMouseData {
        fn client_coordinates(&self) -> ClientPoint {
            ClientPoint::new(self.0.client_x().into(), self.0.client_y().into())
        }
        fn page_coordinates(&self) -> PagePoint {
            PagePoint::new(self.0.page_x().into(), self.0.page_y().into())
        }
        fn screen_coordinates(&self) -> ScreenPoint {
            ScreenPoint::new(self.0.screen_x().into(), self.0.screen_y().into())
        }
    }
    impl InteractionElementOffset for NativeMouseData {
        fn element_coordinates(&self) -> ElementPoint {
            ElementPoint::new(self.0.offset_x().into(), self.0.offset_y().into())
        }
    }
    impl ModifiersInteraction for NativeMouseData {
        fn modifiers(&self) -> Modifiers {
            let mut modifiers = Modifiers::empty();
            modifiers.set(Modifiers::ALT, self.0.alt_key());
            modifiers.set(Modifiers::CONTROL, self.0.ctrl_key());
            modifiers.set(Modifiers::META, self.0.meta_key());
            modifiers.set(Modifiers::SHIFT, self.0.shift_key());
            modifiers
        }
    }
    impl PointerInteraction for NativeMouseData {
        fn trigger_button(&self) -> Option<MouseButton> {
            Some(MouseButton::from_web_code(self.0.button()))
        }
        fn held_buttons(&self) -> MouseButtonSet {
            decode_mouse_button_set(self.0.buttons())
        }
    }

    /// New Dioxus metadata isolates capture caller cancellation from child handlers.
    /// This never reconstructs or redispatches the native event.
    pub fn mouse_event(event: &DomMouseEvent) -> MouseEvent {
        let wrapped = Event::new(
            Rc::new(MouseData::new(NativeMouseData(event.clone()))),
            event.bubbles(),
        );
        if event.default_prevented() {
            wrapped.prevent_default();
        }
        wrapped
    }

    /// Apply ordinary callback cancellation synchronously at the native boundary.
    /// Capture policies may instead inspect this metadata and defer cancellation.
    pub fn apply_cancellation(event: &MouseEvent, native: &DomEvent) {
        if !event.default_action_enabled() {
            native.prevent_default();
        }
        if !event.propagates() {
            native.stop_propagation();
        }
    }

    /// Walk only as far as the owning surface, ignoring read-only controls.
    pub fn has_interactive_ancestor(target: &Element, root: &Element) -> bool {
        let mut current = Some(target.clone());
        while let Some(element) = current {
            if element == *root || element.tag_name() == "BODY" {
                break;
            }
            if (element.matches(INTERACTIVE_SELECTORS).unwrap_or(false)
                && element.get_attribute("aria-readonly").as_deref() != Some("true"))
                || is_layer_surface(&element)
            {
                return true;
            }
            current = element.parent_element();
        }
        false
    }

    /// Open surfaces belong to their layer, not the surrounding card.
    pub fn is_layer_surface(element: &Element) -> bool {
        element.matches("[data-astryx-layer-root],[data-astryx-tooltip-surface], [role=tooltip],dialog[open],[popover]:popover-open").unwrap_or(false)
    }

    /// A selection outside this surface does not suppress its action.
    pub fn has_text_selection(root: &Element) -> bool {
        root.owner_document()
            .and_then(|doc| doc.get_selection().ok().flatten())
            .is_some_and(|selection| {
                !selection.is_collapsed() && root.contains(selection.anchor_node().as_ref())
            })
    }

    /// Shared click rule; component-specific callback ordering stays caller-owned.
    pub fn allows_activation(event: &DomMouseEvent, root: &Element, is_disabled: bool) -> bool {
        !is_disabled
            && !has_text_selection(root)
            && event
                .target()
                .and_then(|target| target.dyn_into::<Element>().ok())
                .is_some_and(|target| !has_interactive_ancestor(&target, root))
    }

    /// Owns a native listener and removes it before releasing its Rust closure.
    pub struct NativeListener {
        target: EventTarget,
        name: String,
        is_capture: bool,
        callback: Closure<dyn FnMut(DomEvent)>,
    }

    impl NativeListener {
        /// Register a synchronous original-event listener at any native boundary.
        pub fn new(
            target: &EventTarget,
            name: &str,
            is_capture: bool,
            callback: impl FnMut(DomEvent) + 'static,
        ) -> Result<Self, wasm_bindgen::JsValue> {
            let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(DomEvent)>);
            target.add_event_listener_with_callback_and_bool(
                name,
                callback.as_ref().unchecked_ref(),
                is_capture,
            )?;
            Ok(Self {
                target: target.clone(),
                name: name.into(),
                is_capture,
                callback,
            })
        }
    }
    impl Drop for NativeListener {
        fn drop(&mut self) {
            let _ = self.target.remove_event_listener_with_callback_and_bool(
                &self.name,
                self.callback.as_ref().unchecked_ref(),
                self.is_capture,
            );
        }
    }

    /// Shared clickable-container click boundary. Policy and callback ordering are
    /// supplied by the owning component; the shared hook owns listener disposal.
    pub fn use_clickable_container(
        selector: String,
        callback: EventHandler<DomEvent>,
    ) -> EventHandler<MountedEvent> {
        use_native_listener(selector, "click", false, callback)
    }

    /// Mount-time listener resource; callbacks update without listener churn.
    /// Call once per event kind in a stable hook order and put the returned handler
    /// on the root's onmounted. Drop removes the listener, including capture listeners.
    pub fn use_native_listener(
        selector: String,
        name: &'static str,
        is_capture: bool,
        callback: EventHandler<DomEvent>,
    ) -> EventHandler<MountedEvent> {
        let mut listener = use_signal(|| None::<NativeListener>);
        use_drop(move || {
            listener.write().take();
        });
        EventHandler::new(move |_| {
            listener.write().take();
            let root = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|doc| doc.query_selector(&selector).ok().flatten());
            if let Some(root) = root {
                let installed =
                    NativeListener::new(root.as_ref(), name, is_capture, move |event| {
                        callback.call(event)
                    });
                if let Ok(installed) = installed {
                    listener.set(Some(installed));
                }
            }
        })
    }
}
