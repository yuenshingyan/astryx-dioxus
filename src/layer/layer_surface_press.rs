// Adapted from Astryx Layer/layerSurfacePress.ts. See THIRD_PARTY_NOTICES.md.
// The existing Tooltip bridge registers its native content/root in this registry.
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

#[wasm_bindgen(
    inline_js = "export function isRegisteredLayerSurface(element) { return document[Symbol.for('astryx.dioxus.layer-surface-press')]?.has(element) === true; }"
)]
extern "C" {
    #[wasm_bindgen(js_name = isRegisteredLayerSurface)]
    fn is_registered_layer_surface(element: &Element) -> bool;
}

pub(crate) fn is_layer_surface(element: &Element) -> bool {
    is_registered_layer_surface(element)
}
