// Adapted from Astryx Layer/layerSurfacePress.ts. Registry ownership stays here.
const layerSurfaceRegistryKey = Symbol.for('astryx.dioxus.layer-surface-press');
const layerSurfaceRegistry = document[layerSurfaceRegistryKey] ||= new WeakSet();
const registerLayerSurface = element => {
  if (!element) return;
  layerSurfaceRegistry.add(element);
  if (element.parentElement?.hasAttribute('popover')) layerSurfaceRegistry.add(element.parentElement);
};
const isLayerSurface = element => layerSurfaceRegistry.has(element);
