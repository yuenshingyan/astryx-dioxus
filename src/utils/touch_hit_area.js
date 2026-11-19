// Adapted from Astryx utils/touchHitArea. See THIRD_PARTY_NOTICES.md.
const heldHitAreaKey = Symbol.for('astryx.dioxus.utils.touch-hit-area');
const holdTouchHitArea = element => {
  const held = document[heldHitAreaKey] || (document[heldHitAreaKey] = new Map());
  let entry = held.get(element);
  if (!entry) {
    const originals = new Map(), written = new Map();
    const deliver = insets => {
      for (const side of ['top', 'right', 'bottom', 'left']) {
        for (const [name, value] of [[`--astryx-hit-inset-${side}`, `${insets[side]}px`], [`--_hit-border-${side}`, `${insets.border?.[side] || 0}px`]]) {
          if (!originals.has(name)) originals.set(name, [element.style.getPropertyValue(name), element.style.getPropertyPriority(name)]);
          if (element.style.getPropertyValue(name) !== value) element.style.setProperty(name, value);
          written.set(name, value);
        }
      }
    };
    const runtime = installHitAreaWatch();
    const stop = runtime.watch(element, deliver);
    const check = event => {
      if (!runtime.accepts(element, event)) { event.preventDefault(); event.stopImmediatePropagation(); }
    };
    element.addEventListener('click', check, true);
    entry = { holders: 0, stop() {
      stop(); element.removeEventListener('click', check, true);
      originals.forEach(([value, priority], name) => {
        if (element.style.getPropertyValue(name) !== written.get(name)) return;
        if (value) element.style.setProperty(name, value, priority);
        else element.style.removeProperty(name);
      });
    } };
    held.set(element, entry);
  }
  entry.holders++;
  let isReleased = false;
  return () => {
    if (isReleased) return;
    isReleased = true; entry.holders--;
    queueMicrotask(() => {
      if (entry.holders || held.get(element) !== entry) return;
      held.delete(element); entry.stop();
      if (!held.size) delete document[heldHitAreaKey];
    });
  };
};
