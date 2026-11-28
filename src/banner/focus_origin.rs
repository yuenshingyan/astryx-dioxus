use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};
use web_sys::{Element, Event, HtmlElement};

use crate::hooks::use_clickable_container::native::NativeListener;

// Only focusin uses this binding. It avoids expanding the shared web-sys features.
#[wasm_bindgen(
    inline_js = "export function bannerRelatedTarget(event) { return event.relatedTarget; }"
)]
extern "C" {
    #[wasm_bindgen(js_name = bannerRelatedTarget)]
    fn related_target(event: &Event) -> JsValue;
}

pub(super) struct FocusOrigin {
    origin: Rc<RefCell<Option<HtmlElement>>>,
    listeners: [NativeListener; 2],
}

impl FocusOrigin {
    pub(super) fn attach(root: &Element) -> Option<Self> {
        let document = root.owner_document()?;
        let origin = Rc::new(RefCell::new(None));
        let listen = |name: &str, candidate: fn(&Event, &Element) -> Option<HtmlElement>| {
            let origin = origin.clone();
            let event_root = root.clone();
            let document = document.clone();
            NativeListener::new(root.as_ref(), name, true, move |event| {
                if !event.default_prevented()
                    && let Some(candidate) = candidate(&event, &event_root)
                    && document.body().as_ref() != Some(&candidate)
                    && !event_root.contains(Some(&candidate))
                {
                    *origin.borrow_mut() = Some(candidate);
                }
            })
            .ok()
        };
        let focus = listen("focusin", |event, _| related_target(event).dyn_into().ok())?;
        // Pointerdown can see the origin before the browser moves (or clears) focus.
        let pointer = listen("pointerdown", |_, root| {
            root.owner_document()?.active_element()?.dyn_into().ok()
        })?;
        Some(Self {
            origin,
            listeners: [focus, pointer],
        })
    }

    pub(super) fn into_origin(self) -> Option<HtmlElement> {
        drop(self.listeners);
        self.origin.borrow_mut().take()
    }
}
