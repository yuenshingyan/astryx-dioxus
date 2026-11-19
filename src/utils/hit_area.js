// Adapted from Astryx utils/hitArea. See THIRD_PARTY_NOTICES.md.
// No input routing or viewport overlay: the owner's CSS band hits its own root.
const hitAreaKey = Symbol.for('astryx.dioxus.utils.hit-area');
const interactiveHitSelector = 'a[href],area[href],button,input:not([type="hidden"]),select,textarea,summary,label[for],[contenteditable=""],[contenteditable="true"],[role="button"],[role="link"],[role="checkbox"],[role="radio"],[role="switch"],[role="tab"],[role="menuitem"],[role="menuitemcheckbox"],[role="menuitemradio"],[role="treeitem"],[role="combobox"],[role="textbox"],[role="searchbox"],[role="spinbutton"],[role="option"],[role="slider"],[tabindex]:not([tabindex="-1"])';
const hitGrowth = rect => ({
  top: Math.max(0, (44 - (rect.bottom - rect.top)) / 2),
  right: Math.max(0, (44 - (rect.right - rect.left)) / 2),
  bottom: Math.max(0, (44 - (rect.bottom - rect.top)) / 2),
  left: Math.max(0, (44 - (rect.right - rect.left)) / 2),
});
const grownHitRect = (rect, inset) => ({ left: rect.left - inset.left, right: rect.right + inset.right, top: rect.top - inset.top, bottom: rect.bottom + inset.bottom });
const hitIntersects = (a, b) => a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom;
const clampedHitInsets = (target, neighbours) => {
  const full = hitGrowth(target), insets = { ...full };
  for (const rect of neighbours) {
    const x = Math.max(rect.left - target.right, target.left - rect.right);
    const y = Math.max(rect.top - target.bottom, target.top - rect.bottom);
    // Existing app-layout overlap cannot be repaired by a hit band. Do not
    // create additional overlap around either already intersecting target.
    if (x < 0 && y < 0) return { top: 0, right: 0, bottom: 0, left: 0 };
    const half = { top: Math.max(0, y) / 2, right: Math.max(0, x) / 2, bottom: Math.max(0, y) / 2, left: Math.max(0, x) / 2 };
    const reach = grownHitRect(target, full), other = grownHitRect(rect, hitGrowth(rect));
    if (!hitIntersects(reach, grownHitRect(rect, half)) && !hitIntersects(other, grownHitRect(target, half)) && !hitIntersects(reach, other)) continue;
    const side = x >= y ? (rect.left >= target.right ? 'right' : 'left') : (rect.top >= target.bottom ? 'bottom' : 'top');
    insets[side] = Math.min(insets[side], half[side]);
  }
  return insets;
};
const installHitAreaWatch = () => {
  if (document[hitAreaKey]) return document[hitAreaKey];
  const watches = new Map(), pointer = matchMedia('(pointer: coarse)');
  let abort = null, resize = null, mutation = null, visibility = null, frame = 0;
  const pending = new Set();
  const visibleTarget = element => {
    const box = element.getBoundingClientRect();
    const css = getComputedStyle(element);
    return element.isConnected && box.width > 0 && box.height > 0 && css.visibility === 'visible' && css.display !== 'none';
  };
  const read = (element, candidates) => {
    const target = element.getBoundingClientRect();
    const sx = target.width / element.offsetWidth || 1, sy = target.height / element.offsetHeight || 1;
    const layout = rect => ({ left: target.left + (rect.left - target.left) / sx, right: target.left + (rect.right - target.left) / sx, top: target.top + (rect.top - target.top) / sy, bottom: target.top + (rect.bottom - target.top) / sy });
    const neighbours = candidates.filter(other => other.element !== element && !other.element.contains(element) && !element.contains(other.element)).map(other => layout(other.rect));
    const css = getComputedStyle(element);
    const border = Object.fromEntries(['top','right','bottom','left'].map(side => [side, parseFloat(css.getPropertyValue(`border-${side}-width`)) || 0]));
    return { ...clampedHitInsets(layout(target), neighbours), border };
  };
  const candidates = () => [...document.querySelectorAll(interactiveHitSelector)].filter(visibleTarget).map(element => ({ element, rect: element.getBoundingClientRect() }));
  const authoredStyle = element => [...element.style].filter(name => !/^--(?:astryx-hit-inset-|_hit-border-)(top|right|bottom|left)$/.test(name))
    .sort().map(name => `${name}:${element.style.getPropertyValue(name)}!${element.style.getPropertyPriority(name)}`).join(';');
  const schedule = () => {
    if (!pointer.matches) return;
    watches.forEach((_, element) => pending.add(element));
    if (!frame) frame = requestAnimationFrame(measure);
  };
  const measure = () => {
    frame = 0;
    if (!pointer.matches || !pending.size) return;
    const targets = candidates(), results = [], start = performance.now();
    for (const element of [...pending]) {
      if (results.length && performance.now() - start > 4) break;
      pending.delete(element);
      const watch = watches.get(element);
      if (!watch || !watch.isVisible || !visibleTarget(element)) continue;
      // Finite moving ancestors/siblings settle before taking layout insets.
      const moving = (element.parentElement?.getAnimations?.({ subtree: true }) || []).filter(animation => ['running', 'paused'].includes(animation.playState) && Number.isFinite(animation.effect?.getComputedTiming().endTime));
      if (moving.length) {
        if (!watch.isWaiting) {
          watch.isWaiting = true;
          Promise.allSettled(moving.map(animation => animation.finished)).then(() => { watch.isWaiting = false; if (watches.has(element)) schedule(); });
        }
        continue;
      }
      results.push([watch, read(element, targets)]);
    }
    results.forEach(([watch, insets]) => watch.deliver(insets));
    if (pending.size) frame = requestAnimationFrame(measure);
  };
  const trackParent = (element, watch) => {
    const previous = watch.parent;
    watch.parent = element.parentElement;
    if (watch.parent) resize?.observe(watch.parent);
    if (previous && previous !== watch.parent && !watches.has(previous) && ![...watches.values()].some(other => other.parent === previous)) resize?.unobserve(previous);
  };
  const stop = () => {
    abort?.abort(); abort = null;
    resize?.disconnect(); mutation?.disconnect(); visibility?.disconnect();
    resize = mutation = visibility = null;
    cancelAnimationFrame(frame); frame = 0; pending.clear();
  };
  const change = () => {
    stop();
    if (!pointer.matches) { watches.forEach(watch => watch.deliver({ top: 0, right: 0, bottom: 0, left: 0 })); return; }
    abort = new AbortController();
    const signal = abort.signal;
    resize = new ResizeObserver(schedule);
    mutation = new MutationObserver(changes => {
      // Ignore the inset writes themselves; style/DOM/name changes elsewhere
      // can move any neighbour, even without resizing the watched target.
      if (changes.some(change => change.type === 'childList')) watches.forEach((watch, element) => trackParent(element, watch));
      if (changes.some(change => {
        if (change.type !== 'attributes' || !watches.has(change.target) || change.attributeName !== 'style') return true;
        const watch = watches.get(change.target), next = authoredStyle(change.target);
        if (watch.authoredStyle === next) return false;
        watch.authoredStyle = next; return true;
      })) schedule();
    });
    mutation.observe(document.documentElement, { subtree: true, childList: true, characterData: true, attributes: true, attributeFilter: ['class', 'style', 'hidden', 'inert', 'role', 'href', 'tabindex', 'disabled', 'aria-disabled'] });
    visibility = new IntersectionObserver(entries => {
      entries.forEach(entry => { const watch = watches.get(entry.target); if (watch) watch.isVisible = entry.isIntersecting; });
      schedule();
    });
    watches.forEach((watch, element) => { resize.observe(element); trackParent(element, watch); visibility.observe(element); });
    window.addEventListener('resize', schedule, { signal, passive: true });
    document.addEventListener('scroll', schedule, { signal, capture: true, passive: true });
    document.addEventListener('animationend', schedule, { signal, capture: true });
    document.addEventListener('transitionend', schedule, { signal, capture: true });
    schedule();
  };
  pointer.addEventListener('change', change);
  const api = {
    watch(element, deliver) {
      const watch = { deliver, isVisible: true, isWaiting: false, parent: null, authoredStyle: authoredStyle(element) };
      watches.set(element, watch);
      if (!abort && pointer.matches) change();
      else if (pointer.matches) { resize.observe(element); trackParent(element, watch); visibility.observe(element); schedule(); }
      return () => {
        watches.delete(element); pending.delete(element); visibility?.unobserve(element);
        if (![...watches.values()].some(other => other.parent === element)) resize?.unobserve(element);
        if (watch.parent && !watches.has(watch.parent) && ![...watches.values()].some(other => other.parent === watch.parent)) resize?.unobserve(watch.parent);
        if (!watches.size) { stop(); pointer.removeEventListener('change', change); delete document[hitAreaKey]; }
      };
    },
    // Recheck the CSS band's click against fresh geometry; never redirect it
    // to another control, consume an input action, or scan on pointermove.
    accepts(element, event) {
      if (!pointer.matches || event.detail === 0) return true;
      const box = element.getBoundingClientRect();
      if (event.clientX >= box.left && event.clientX <= box.right && event.clientY >= box.top && event.clientY <= box.bottom) return true;
      const insets = read(element, candidates());
      watches.get(element)?.deliver(insets);
      const sx = box.width / element.offsetWidth || 1, sy = box.height / element.offsetHeight || 1;
      return event.clientX >= box.left - insets.left * sx && event.clientX <= box.right + insets.right * sx && event.clientY >= box.top - insets.top * sy && event.clientY <= box.bottom + insets.bottom * sy;
    },
  };
  document[hitAreaKey] = api;
  return api;
};
