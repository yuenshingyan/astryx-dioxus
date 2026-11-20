// Adapted from Astryx Layer/layerSurfacePress.ts. Registry ownership stays here.
const layerSurfaceRegistryKey = Symbol.for('astryx.dioxus.layer-surface-press');
const layerSurfaceOwnerKey = Symbol.for('astryx.dioxus.layer-surface-owner');
const layerSurfaceRegistry = document[layerSurfaceRegistryKey] ||= new WeakSet();
const layerSurfaceOwners = document[layerSurfaceOwnerKey] ||= new WeakMap();
// Neutral native-portal metadata replaces the logical ancestry supplied by React.
const registerLayerSurface = (element, anchor) => {
  if (!element) return;
  layerSurfaceRegistry.add(element);
  if (anchor) layerSurfaceOwners.set(element, anchor);
  if (element.parentElement?.hasAttribute('popover')) {
    layerSurfaceRegistry.add(element.parentElement);
    if (anchor) layerSurfaceOwners.set(element.parentElement, anchor);
  }
};
const clearLayerSurfaceOwner = element => {
  if (!element) return;
  layerSurfaceOwners.delete(element);
  if (element.parentElement?.hasAttribute('popover')) layerSurfaceOwners.delete(element.parentElement);
};
const isLayerSurface = element => layerSurfaceRegistry.has(element);
