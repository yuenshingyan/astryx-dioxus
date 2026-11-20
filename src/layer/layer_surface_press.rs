// Adapted from Astryx Layer/layerSurfacePress.ts. See THIRD_PARTY_NOTICES.md.
// The existing Tooltip bridge registers its native content/root in this registry.
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

#[wasm_bindgen(
    inline_js = "export function isRegisteredLayerSurface(element) { return document[Symbol.for('astryx.dioxus.layer-surface-press')]?.has(element) === true; } export function layerSurfaceOwner(element) { return document[Symbol.for('astryx.dioxus.layer-surface-owner')]?.get(element) ?? null; }"
)]
extern "C" {
    #[wasm_bindgen(js_name = isRegisteredLayerSurface)]
    fn is_registered_layer_surface(element: &Element) -> bool;
    #[wasm_bindgen(js_name = layerSurfaceOwner)]
    pub(crate) fn layer_surface_owner(element: &Element) -> Option<Element>;
}

pub(crate) fn is_layer_surface(element: &Element) -> bool {
    is_registered_layer_surface(element)
}
