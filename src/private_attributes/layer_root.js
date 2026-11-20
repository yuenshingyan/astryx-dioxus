// Adapted from Astryx privateAttributes/layerRoot.stylex.ts.
const layerRootAttribute = 'data-astryx-layer-root';
const markLayerRoot = element => element.setAttribute(layerRootAttribute, '');
const isLayerRoot = element => element.hasAttribute(layerRootAttribute);
const closestLayerRoot = element => element?.closest?.(`[${layerRootAttribute}]`) || null;
