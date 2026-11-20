// Required centered-anchor subset of Layer/useLayer and layerSlideRules at the pinned source.
// Native anchor layout, not a JS approximation, owns scrolling, flipping and clipping.
export const createLayerPresentation = (element, tip, placement) => {
  const name = `--astryx-truncation-${tip.id.replace(/[^a-zA-Z0-9_-]/g, '-')}`;
  const names = () => (element.style.getPropertyValue('anchor-name') || '').split(',').map(x => x.trim()).filter(x => x && x !== 'none');
  const add = () => element.style.setProperty('anchor-name', [...new Set([...names(), name])].join(', '));
  const remove = () => {
    const remaining = names().filter(x => x !== name);
    if (remaining.length) element.style.setProperty('anchor-name', remaining.join(', '));
    else element.style.removeProperty('anchor-name');
  };
  tip.style.positionAnchor = name;
  tip.dataset.placement = placement;
  let generation = 0;
  return {
    show() {
      generation++;
      add();
      tip.showPopover();
      // The entrance follows the actual landing side, including a native flip.
      const a = element.getBoundingClientRect(), b = tip.getBoundingClientRect();
      const x = b.right <= a.left ? 1 : b.left >= a.right ? -1 : 0;
      const y = b.bottom <= a.top ? 1 : b.top >= a.bottom ? -1 : 0;
      tip.style.setProperty('--_astryx-layer-enter-x', x);
      tip.style.setProperty('--_astryx-layer-enter-y', y);
      tip.style.setProperty('--_astryx-layer-enter-origin', `${x === 1 ? '100%' : x === -1 ? '0%' : '50%'} ${y === 1 ? '100%' : y === -1 ? '0%' : '50%'}`);
      const gap = x === 1 ? a.left - b.right : x === -1 ? b.left - a.right : y === 1 ? a.top - b.bottom : b.top - a.bottom;
      tip.style.setProperty('--_astryx-layer-enter-gap', `${Math.max(0, gap)}px`);
    },
    hide() {
      const current = ++generation;
      // Source has no default exit animation. A consumer's discrete display exit
      // can keep painting; keep its anchor until its finite animations finish.
      tip.hidePopover();
      const animations = tip.getAnimations({subtree: true}).filter(a => a.playState === 'running' && a.effect?.getComputedTiming().endTime !== Infinity);
      if (!animations.length) { remove(); return; }
      Promise.allSettled(animations.map(a => a.finished)).then(() => { if (generation === current) remove(); });
    },
    dispose() { generation++; remove(); tip.remove(); },
  };
};
