// SelectableCard's released before-layer policy. Neutral transport is shared;
// isolation and delayed native default cancellation stay with their source owner.
use dioxus::prelude::{EventHandler, MouseEvent, PointerEvent};
use wasm_bindgen::{prelude::*, throw_val};
use web_sys::{
    Element, HtmlInputElement, MouseEvent as DomMouseEvent, PointerEvent as DomPointerEvent,
};

use crate::hooks::use_clickable_container::native::with_current_target;
use crate::layer::layer_surface_press::layer_surface_owner;
use crate::private_attributes::layer_root::closest_layer_root;

#[wasm_bindgen(inline_js = r#"
export function selectableDeferDefault(event, root) {
  const doc = root.ownerDocument;
  const type = event.type;
  const cancel = bubbled => {
    doc.removeEventListener(type, cancel);
    if (bubbled === event) event.preventDefault();
  };
  doc.addEventListener(type, cancel);
  return { doc, type, cancel };
}
export function selectableDisposeDefault(token) {
  token.doc.removeEventListener(token.type, token.cancel);
}
export function selectableRestoreRadioPeers(input) {
  // A rejected native radio click has also unchecked its selected sibling.
  // Repair only this source-owned controlled channel, in the same native
  // group (name, form owner and tree root), not arbitrary product inputs.
  if (!input.name) return;
  for (const peer of input.getRootNode().querySelectorAll('.astryx-selectable-card__input[type="radio"]')) {
    if (peer.name === input.name && peer.form === input.form) {
      peer.checked = peer.parentElement.dataset.selected === 'true';
    }
  }
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = selectableDeferDefault)]
    fn defer_default(event: &DomMouseEvent, root: &Element) -> JsValue;
    #[wasm_bindgen(js_name = selectableDisposeDefault)]
    fn dispose_default(token: &JsValue);
    #[wasm_bindgen(js_name = selectableRestoreRadioPeers)]
    fn restore_radio_peers(input: &HtmlInputElement);
}

pub(super) fn call_mouse_owner(
    handler: EventHandler<MouseEvent>,
    event: MouseEvent,
    root: &Element,
) {
    let data = event.data();
    if let Some(native) = data.downcast::<DomMouseEvent>() {
        if let Err(err) = with_current_target(native.as_ref(), root, move || handler.call(event)) {
            throw_val(err);
        }
    } else {
        handler.call(event);
    }
}

pub(super) fn call_pointer_owner(
    handler: EventHandler<PointerEvent>,
    event: PointerEvent,
    root: &Element,
) {
    let data = event.data();
    if let Some(native) = data.downcast::<DomPointerEvent>() {
        if let Err(err) = with_current_target(native.as_ref(), root, move || handler.call(event)) {
            throw_val(err);
        }
    } else {
        handler.call(event);
    }
}

pub(super) fn owns_portaled_layer(root: &Element, layer: &Element) -> bool {
    let mut current = Some(layer.clone());
    let mut visited = Vec::new();
    while let Some(layer) = current {
        if visited.contains(&layer) {
            return false;
        }
        visited.push(layer.clone());
        let Some(anchor) = layer_surface_owner(&layer) else {
            return false;
        };
        if root.contains(Some(&anchor)) {
            return true;
        }
        // A trigger may itself live in a parent portal. Follow only actual
        // Layer-owned metadata; arbitrary popover/dialog attributes never count.
        current = closest_layer_root(&anchor);
    }
    false
}

pub(super) fn restore_checked(input: &HtmlInputElement, is_selected: bool) {
    if input.type_() == "radio" {
        restore_radio_peers(input);
    }
    input.set_checked(is_selected);
}

pub(super) struct DeferredDefault(JsValue);
impl DeferredDefault {
    pub fn new(event: &DomMouseEvent, root: &Element) -> Self {
        Self(defer_default(event, root))
    }
}
impl Drop for DeferredDefault {
    fn drop(&mut self) {
        dispose_default(&self.0);
    }
}
