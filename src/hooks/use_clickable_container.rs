// Adapted from Astryx hooks/useClickableContainer.ts. See THIRD_PARTY_NOTICES.md.
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::Element;

/// Nested controls own their activation, including marked nested containers.
#[cfg(target_arch = "wasm32")]
pub(crate) const INTERACTIVE_SELECTORS: &str = "button,a,input,select,textarea,[role=button],[role=link],[role=checkbox],[role=radio],[role=switch],[role=tab],[role=menuitem],[role=option],[role=combobox],[role=listbox],[role=slider],[role=spinbutton],[data-pressable-container]";

/// Renderer-neutral owner ref used by the native event adapter.
#[derive(Clone, Default)]
pub(crate) struct NativeOwner {
    #[cfg(target_arch = "wasm32")]
    element: Rc<RefCell<Option<Element>>>,
}

impl NativeOwner {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn element(&self) -> Option<Element> {
        self.element.borrow().clone()
    }
}

/// Adapt a native mounted ref without selecting a renderer for host rendering.
pub(crate) fn use_native_owner() -> (NativeOwner, EventHandler<MountedEvent>) {
    let owner = use_hook(NativeOwner::default);
    let mounted_owner = owner.clone();
    let mounted = EventHandler::new(move |event: MountedEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            *mounted_owner.element.borrow_mut() = event.data().downcast::<Element>().cloned();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (&mounted_owner, event);
        }
    });
    (owner, mounted)
}

/// Native event boundary. No renderer or DOM is required for host rendering.
#[cfg(target_arch = "wasm32")]
pub(crate) mod native {
    use std::any::Any;
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::rc::Rc;

    use dioxus::html::geometry::{ClientPoint, ElementPoint, PagePoint, ScreenPoint};
    use dioxus::html::input_data::{MouseButton, MouseButtonSet, decode_mouse_button_set};
    use dioxus::html::{HasMouseData, HasPointerData, Modifiers};
    use dioxus::prelude::*;
    use wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen};
    use web_sys::{
        Document, Element, Event as DomEvent, EventTarget, HtmlElement, MouseEvent as DomMouseEvent,
    };

    use super::INTERACTIVE_SELECTORS;
    pub(crate) use crate::layer::layer_surface_press::is_layer_surface;
    use crate::private_attributes::layer_root::closest_layer_root;
    use crate::utils::safe_url::is_safe_url;

    #[wasm_bindgen(
        inline_js = "export function withOwnerCurrentTarget(event, owner, callback) { const previous = Object.getOwnPropertyDescriptor(event, 'currentTarget'); Object.defineProperty(event, 'currentTarget', { configurable: true, get: () => owner }); try { callback(); } finally { if (previous) Object.defineProperty(event, 'currentTarget', previous); else delete event.currentTarget; } }"
    )]
    extern "C" {
        #[wasm_bindgen(catch, js_name = withOwnerCurrentTarget)]
        fn with_owner_current_target(
            event: &DomEvent,
            owner: &Element,
            callback: &Closure<dyn FnMut()>,
        ) -> Result<(), JsValue>;
    }

    /// React's callback-local owner adapted over the SAME original DOM event.
    /// Ordinary callers must retain their original Dioxus Event metadata. The
    /// exact prior own descriptor (or absence) is restored in JS try/finally,
    /// including nested scopes and JavaScript exceptions. No target/trust/event
    /// reconstruction or renderer-private access is involved.
    pub fn with_current_target(
        event: &DomEvent,
        owner: &Element,
        callback: impl FnOnce() + 'static,
    ) -> Result<(), JsValue> {
        let mut callback = Some(callback);
        let callback = Closure::wrap(Box::new(move || {
            if let Some(callback) = callback.take() {
                callback();
            }
        }) as Box<dyn FnMut()>);
        with_owner_current_target(event, owner, &callback)
    }

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

    /// Native pointer data with the same downcast contract as native mouse data.
    pub struct NativePointerData(pub web_sys::PointerEvent);
    impl InteractionLocation for NativePointerData {
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
    impl InteractionElementOffset for NativePointerData {
        fn element_coordinates(&self) -> ElementPoint {
            ElementPoint::new(self.0.offset_x().into(), self.0.offset_y().into())
        }
    }
    impl ModifiersInteraction for NativePointerData {
        fn modifiers(&self) -> Modifiers {
            NativeMouseData(self.0.clone().unchecked_into()).modifiers()
        }
    }
    impl PointerInteraction for NativePointerData {
        fn trigger_button(&self) -> Option<MouseButton> {
            Some(MouseButton::from_web_code(self.0.button()))
        }
        fn held_buttons(&self) -> MouseButtonSet {
            decode_mouse_button_set(self.0.buttons())
        }
    }
    impl HasPointerData for NativePointerData {
        fn pointer_id(&self) -> i32 {
            self.0.pointer_id()
        }
        fn width(&self) -> f64 {
            self.0.width().into()
        }
        fn height(&self) -> f64 {
            self.0.height().into()
        }
        fn pressure(&self) -> f32 {
            self.0.pressure()
        }
        fn tangential_pressure(&self) -> f32 {
            self.0.tangential_pressure()
        }
        fn tilt_x(&self) -> i32 {
            self.0.tilt_x()
        }
        fn tilt_y(&self) -> i32 {
            self.0.tilt_y()
        }
        fn twist(&self) -> i32 {
            self.0.twist()
        }
        fn pointer_type(&self) -> String {
            self.0.pointer_type()
        }
        fn is_primary(&self) -> bool {
            self.0.is_primary()
        }
        fn as_any(&self) -> &dyn Any {
            &self.0
        }
    }

    /// Build separate metadata while preserving the original native pointer event.
    pub fn pointer_event(event: &web_sys::PointerEvent) -> PointerEvent {
        let wrapped = Event::new(
            Rc::new(PointerData::new(NativePointerData(event.clone()))),
            event.bubbles(),
        );
        if event.default_prevented() {
            wrapped.prevent_default();
        }
        wrapped
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
    pub fn apply_cancellation<T: ?Sized>(event: &Event<T>, native: &DomEvent) {
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
                || closest_layer_root(&element).is_some_and(|layer| layer == element)
            {
                return true;
            }
            current = element.parent_element();
        }
        false
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

    /// Source-prescribed no-callback delegation to the owning native button.
    /// This is a native activation, not transport for a reconstructed Rust callback.
    pub fn delegate_action(event: &DomMouseEvent, interactive: &HtmlElement) {
        let options = web_sys::MouseEventInit::new();
        options.set_bubbles(event.bubbles());
        options.set_cancelable(event.cancelable());
        options.set_ctrl_key(event.ctrl_key());
        options.set_meta_key(event.meta_key());
        options.set_shift_key(event.shift_key());
        options.set_alt_key(event.alt_key());
        options.set_button(event.button());
        if let Ok(delegated) = DomMouseEvent::new_with_mouse_event_init_dict("click", &options) {
            let _ = interactive.dispatch_event(&delegated);
            event.stop_propagation();
        }
    }

    /// Select the source-prescribed navigation sink only after shared URL acceptance.
    /// Middle clicks open separately and never delegate or invoke an action callback.
    pub fn navigate(
        event: &DomMouseEvent,
        href: &str,
        target: Option<&str>,
        interactive: Option<&HtmlElement>,
    ) {
        if !is_safe_url(href) || (event.button() != 1 && event.default_prevented()) {
            return;
        }
        if event.button() == 1 || target == Some("_blank") || event.ctrl_key() || event.meta_key() {
            if let Some(window) = web_sys::window() {
                let _ = window.open_with_url_and_target_and_features(href, "_blank", "noopener");
            }
        } else if let Some(interactive) = interactive {
            interactive.click();
        } else if let Some(window) = web_sys::window() {
            let _ = window.location().set_href(href);
        }
    }

    /// Owns a native listener and removes it before releasing its Rust closure.
    pub struct NativeListener {
        target: EventTarget,
        name: String,
        is_capture: bool,
        callback: Closure<dyn Fn(DomEvent)>,
    }

    impl NativeListener {
        /// Register a synchronous original-event listener at any native boundary.
        pub fn new(
            target: &EventTarget,
            name: &str,
            is_capture: bool,
            callback: impl Fn(DomEvent) + 'static,
        ) -> Result<Self, wasm_bindgen::JsValue> {
            let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn(DomEvent)>);
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
        callback: Rc<dyn Fn(DomEvent)>,
    ) -> EventHandler<MountedEvent> {
        use_native_event_listener(selector, "click", false, callback)
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
        use_native_event_listener(
            selector,
            name,
            is_capture,
            Rc::new(move |event| callback.call(event)),
        )
    }

    /// Reentrant native transport, used when a handler synchronously delegates a
    /// click. The callback slot is cloned before invocation, never borrowed across
    /// application code or native dispatch. No event classification policy is added.
    pub fn use_native_event_listener(
        selector: String,
        name: &'static str,
        is_capture: bool,
        callback: Rc<dyn Fn(DomEvent)>,
    ) -> EventHandler<MountedEvent> {
        let callback_slot = use_hook(|| Rc::new(RefCell::new(callback.clone())));
        *callback_slot.borrow_mut() = callback;
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
                let callback_slot = callback_slot.clone();
                let installed =
                    NativeListener::new(root.as_ref(), name, is_capture, move |event| {
                        let callback = callback_slot.borrow().clone();
                        callback(event);
                    });
                if let Ok(installed) = installed {
                    listener.set(Some(installed));
                }
            }
        })
    }

    type NativeCallbackSlot = Rc<RefCell<Rc<dyn Fn(DomEvent)>>>;

    #[derive(Clone)]
    struct DocumentRegistration {
        token: Rc<()>,
        root: Element,
        callback: NativeCallbackSlot,
    }
    struct DocumentListenerGroup {
        document: Document,
        name: &'static str,
        is_capture: bool,
        registrations: Rc<RefCell<Vec<DocumentRegistration>>>,
        listener: NativeListener,
    }
    thread_local! {
        static DOCUMENT_LISTENERS: RefCell<Vec<DocumentListenerGroup>> = const { RefCell::new(Vec::new()) };
    }
    struct DocumentSubscription {
        document: Document,
        name: &'static str,
        is_capture: bool,
        token: Rc<()>,
    }
    impl Drop for DocumentSubscription {
        fn drop(&mut self) {
            DOCUMENT_LISTENERS.with(|groups| {
                let mut groups = groups.borrow_mut();
                let Some(index) = groups.iter().position(|group| {
                    group.document == self.document
                        && group.name == self.name
                        && group.is_capture == self.is_capture
                }) else {
                    return;
                };
                let is_empty = {
                    let mut registrations = groups[index].registrations.borrow_mut();
                    registrations
                        .retain(|registration| !Rc::ptr_eq(&registration.token, &self.token));
                    registrations.is_empty()
                };
                if is_empty {
                    let DocumentListenerGroup { listener, .. } = groups.remove(index);
                    drop(listener);
                }
            });
        }
    }
    fn subscribe_document(
        root: Element,
        name: &'static str,
        is_capture: bool,
        callback: NativeCallbackSlot,
    ) -> Result<DocumentSubscription, JsValue> {
        let document = root
            .owner_document()
            .ok_or_else(|| JsValue::from_str("Native event owner has no document"))?;
        let token = Rc::new(());
        DOCUMENT_LISTENERS.with(|groups| {
            let mut groups = groups.borrow_mut();
            let index = if let Some(index) = groups.iter().position(|group| {
                group.document == document && group.name == name && group.is_capture == is_capture
            }) {
                index
            } else {
                let registrations = Rc::new(RefCell::new(Vec::<DocumentRegistration>::new()));
                let routed = registrations.clone();
                let listener =
                    NativeListener::new(document.as_ref(), name, is_capture, move |event| {
                        let mut ordered = routed.borrow().clone();
                        ordered.sort_by(|a, b| {
                            if a.root == b.root {
                                return Ordering::Equal;
                            }
                            let order = if a.root.contains(Some(&b.root)) {
                                Ordering::Less
                            } else if b.root.contains(Some(&a.root)) {
                                Ordering::Greater
                            } else {
                                Ordering::Equal
                            };
                            if is_capture { order } else { order.reverse() }
                        });
                        for registration in ordered {
                            let is_active = routed
                                .borrow()
                                .iter()
                                .any(|active| Rc::ptr_eq(&active.token, &registration.token));
                            if is_active {
                                let callback = registration.callback.borrow().clone();
                                callback(event.clone());
                            }
                        }
                    })?;
                let index = groups.len();
                groups.push(DocumentListenerGroup {
                    document: document.clone(),
                    name,
                    is_capture,
                    registrations,
                    listener,
                });
                index
            };
            groups[index]
                .registrations
                .borrow_mut()
                .push(DocumentRegistration {
                    token: token.clone(),
                    root,
                    callback,
                });
            Ok(DocumentSubscription {
                document,
                name,
                is_capture,
                token,
            })
        })
    }

    /// A shared owner-document boundary using the REAL mounted element's document.
    /// Attach the returned handler to the owning root's onmounted. Capture routes
    /// parent-before-child (bubble routes child-before-parent), independently of
    /// mount order. Every callback decides its own event/policy applicability.
    /// No borrow survives application code; dropped subscriptions are skipped
    /// even within an in-flight dispatch. Last drop removes the native listener.
    pub fn use_native_document_listener(
        name: &'static str,
        is_capture: bool,
        callback: EventHandler<DomEvent>,
    ) -> EventHandler<MountedEvent> {
        let callback: Rc<dyn Fn(DomEvent)> = Rc::new(move |event| callback.call(event));
        let slot = use_hook(|| Rc::new(RefCell::new(callback.clone())));
        *slot.borrow_mut() = callback;
        let mut subscription = use_signal(|| None::<DocumentSubscription>);
        use_drop(move || {
            subscription.write().take();
        });
        EventHandler::new(move |event: MountedEvent| {
            subscription.write().take();
            if let Some(root) = event.data().downcast::<Element>().cloned()
                && let Ok(installed) = subscribe_document(root, name, is_capture, slot.clone())
            {
                subscription.set(Some(installed));
            }
        })
    }
}
