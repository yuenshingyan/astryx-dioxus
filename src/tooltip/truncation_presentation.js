// Private demand-loaded Tooltip surface. Text owns measurement and interest;
// Layer owns geometry. This module is deliberately not included in the eval bridge.
export const createTooltipPresentation = ({record, tip, createLayerPresentation}) => {
  tip.className = 'astryx-tooltip astryx-truncation-tooltip';
  const properties = ['--color-background-inverted', '--color-background-surface', '--radius-container', '--font-family-body', '--text-body-size', '--text-body-leading', '--text-body-weight', '--spacing-1', '--spacing-2', '--spacing-4', '--duration-fast-max', '--ease-standard', '--astryx-layer-inset-block-start', '--astryx-layer-inset-block-end', '--astryx-layer-inset-inline-start', '--astryx-layer-inset-inline-end'];
  const refresh = () => {
    const css = getComputedStyle(record.element);
    properties.forEach(name => { const value = css.getPropertyValue(name); if (value) tip.style.setProperty(name, value); else tip.style.removeProperty(name); });
    tip.style.direction = css.direction;
    tip.style.colorScheme = css.colorScheme;
    tip.dataset.placement = record.element.getAttribute('data-astryx-truncation-placement') || 'above';
  };
  refresh();
  const content = document.createElement('div');
  content.className = 'astryx-truncation-tooltip__content';
  content.textContent = tip.textContent;
  tip.replaceChildren(content);
  // Preserve native modal isolation and inherited theme through an explicit copy.
  const modal = record.element.closest('dialog:modal');
  if (modal) modal.append(tip);
  const layer = createLayerPresentation(record.element, tip, record.element.getAttribute('data-astryx-truncation-placement') || 'above');
  return { ...layer, update(text) { content.textContent = text; refresh(); } };
};
