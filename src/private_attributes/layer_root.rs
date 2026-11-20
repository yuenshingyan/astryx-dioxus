// Adapted from Astryx privateAttributes/layerRoot.stylex.ts.
// See THIRD_PARTY_NOTICES.md. Unmarked native dialogs/popovers are not layers.
use web_sys::Element;

const LAYER_ROOT_SELECTOR: &str = "[data-astryx-layer-root]";

pub(crate) fn closest_layer_root(target: &Element) -> Option<Element> {
    target.closest(LAYER_ROOT_SELECTOR).ok().flatten()
}
