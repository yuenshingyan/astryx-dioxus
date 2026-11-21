const [id, initial] = await dioxus.recv();
const viewport = document.querySelector(`[data-astryx-scroll-viewport="${CSS.escape(id)}"]`);
const content = viewport?.querySelector(':scope > [data-scroll-content]');
if (!content) return;
let config = initial;
let frame;
let mapping;
const set = (name, value) => {
  if (value === null) { if (viewport.hasAttribute(name)) viewport.removeAttribute(name); }
  else if (viewport.getAttribute(name) !== value) viewport.setAttribute(name, value);
};
const publish = () => {
  if (!mapping) return;
  let effective = false;
  for (const [axis, physical, requested] of mapping) {
    const extent = physical === 'x' ? viewport.scrollWidth - viewport.clientWidth : viewport.scrollHeight - viewport.clientHeight;
    const offset = Math.abs(physical === 'x' ? viewport.scrollLeft : viewport.scrollTop);
    const scrollable = requested && extent > 0;
    effective ||= scrollable;
    set(`data-scrollable-${axis}`, scrollable ? 'true' : null);
    set(`data-scroll-${axis}-start`, !scrollable || offset < 1 ? 'true' : null);
    set(`data-scroll-${axis}-end`, !scrollable || offset >= extent - 1 ? 'true' : null);
  }
  set('tabindex', effective ? '0' : document.activeElement === viewport ? '-1' : null);
};
const measure = () => {
  frame = undefined;
  if (!viewport.isConnected || !viewport.clientWidth || !viewport.clientHeight) return;
  const [axis, always, contain] = config;
  const vertical = getComputedStyle(viewport).writingMode !== 'horizontal-tb';
  mapping = [['inline', vertical ? 'y' : 'x', axis !== 'block'], ['block', vertical ? 'x' : 'y', axis !== 'inline']];
  const overflow = {x: false, y: false};
  const requested = {x: false, y: false};
  for (const [, physical, enabled] of mapping) {
    requested[physical] = enabled;
    overflow[physical] = enabled && (physical === 'x' ? viewport.scrollWidth > viewport.clientWidth : viewport.scrollHeight > viewport.clientHeight);
  }
  const contains = always || overflow.x || overflow.y;
  const values = ['x', 'y'].map(key => !contains ? 'clip' : (always ? requested[key] : overflow[key]) ? 'auto' : 'hidden');
  set('data-overflow-x', values[0]);
  set('data-overflow-y', values[1]);
  set('data-contain-x', contain && overflow.x ? 'true' : null);
  set('data-contain-y', contain && overflow.y ? 'true' : null);
  publish();
};
const schedule = () => { if (frame === undefined) frame = requestAnimationFrame(measure); };
const resize = new ResizeObserver(schedule);
resize.observe(viewport);
resize.observe(content);
const mutations = new MutationObserver(schedule);
mutations.observe(content, {subtree: true, childList: true, attributes: true, characterData: true});
for (let ancestor = viewport; ancestor; ancestor = ancestor.parentElement) {
  mutations.observe(ancestor, {attributes: true, attributeFilter: ['style', 'dir', 'class', 'hidden']});
}
viewport.addEventListener('scroll', publish, {passive: true});
viewport.addEventListener('focusout', schedule);
viewport.addEventListener('load', schedule, true);
try {
  measure();
  while (true) {
    const next = await dioxus.recv();
    if (next === false) break;
    config = next;
    schedule();
  }
} finally {
  cancelAnimationFrame(frame);
  resize.disconnect();
  mutations.disconnect();
  viewport.removeEventListener('scroll', publish);
  viewport.removeEventListener('focusout', schedule);
  viewport.removeEventListener('load', schedule, true);
}
