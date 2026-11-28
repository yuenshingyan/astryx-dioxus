const owner = await dioxus.recv();
const strip = document.querySelector(`[data-astryx-tab-list-owner="${CSS.escape(owner)}"]`);
if (!strip) return;

const stop = '.astryx-tab[data-tab-value]';
const boundary = '[data-astryx-tab-list-owner], [popover]';
const ownsStop = item => item?.matches(stop) && item.closest(boundary) === strip;
const children = new Map();
let frame;
let previousFocus;
let previousSelection;
let previousWidth;
let previousContentWidth;
let canFade = true;

const leftFade = 'data-tab-overflow-left';
const rightFade = 'data-tab-overflow-right';
const updateFades = () => {
  const extent = strip.scrollWidth - strip.clientWidth;
  const rtl = getComputedStyle(strip).direction === 'rtl';
  // RTL scrollLeft is negative; clamp elastic overscroll at both ends.
  const offset = Math.min(extent, Math.max(0, rtl ? -strip.scrollLeft : strip.scrollLeft));
  const hasStart = offset > 1;
  const hasEnd = offset < extent - 1;
  strip.toggleAttribute(leftFade, canFade && (rtl ? hasEnd : hasStart));
  strip.toggleAttribute(rightFade, canFade && (rtl ? hasStart : hasEnd));
};

const reveal = (item, inset, rtl) => {
  const rect = item.getBoundingClientRect();
  const left = strip.getBoundingClientRect().left + strip.clientLeft;
  const pastLeft = rect.left - (left + inset);
  const pastRight = rect.right - (left + strip.clientWidth - inset);
  // For a stop that cannot fit, show its reading start.
  const delta = rect.width > strip.clientWidth - 2 * inset
    ? (rtl ? pastRight : pastLeft)
    : pastLeft < 0 ? pastLeft : Math.max(0, pastRight);
  if (delta) strip.scrollBy({ left: delta, behavior: 'instant' });
};

const measure = () => {
  frame = undefined;
  const directChildren = new Set(strip.children);
  for (const [child, release] of children) {
    if (!directChildren.has(child)) {
      release();
      children.delete(child);
    }
  }
  for (const child of directChildren) {
    if (!children.has(child)) children.set(child, observeResize(child, schedule));
  }
  const active = document.activeElement;
  const focused = ownsStop(active) ? active : null;
  const selected = Array.from(strip.querySelectorAll(`${stop}:is([aria-current="true"], [aria-selected="true"])`))
    .find(ownsStop);
  const width = strip.clientWidth;
  const contentWidth = strip.scrollWidth;
  const style = getComputedStyle(strip);
  const inset = Number.parseFloat(style.scrollPaddingLeft) || 0;
  const padding = Number.parseFloat(style.paddingLeft) || 0;
  // A mask must not obscure the focus ring when the stop cannot clear both fades.
  canFade = !focused || focused.getBoundingClientRect().width + 2 * inset <= width;
  const target = focused ?? selected;
  const hasNewFocus = focused !== null && focused !== previousFocus;
  const hasNewSelection = selected !== previousSelection;
  const hasResized = width !== previousWidth || contentWidth !== previousContentWidth;
  if (target && contentWidth > width && (hasNewFocus || hasNewSelection || hasResized)) {
    reveal(target, canFade ? inset : padding, style.direction === 'rtl');
  }
  previousFocus = focused;
  previousSelection = selected;
  previousWidth = width;
  previousContentWidth = contentWidth;
  updateFades();
};
const schedule = () => { if (frame === undefined) frame = requestAnimationFrame(measure); };
const mutations = new MutationObserver(schedule);
mutations.observe(strip, { subtree: true, childList: true, characterData: true, attributes: true,
  attributeFilter: ['aria-current', 'aria-selected', 'class', 'style', 'dir', 'hidden'] });
for (let ancestor = strip.parentElement; ancestor; ancestor = ancestor.parentElement) {
  mutations.observe(ancestor, { attributes: true, attributeFilter: ['dir'] });
}
const releaseStrip = observeResize(strip, schedule);
strip.addEventListener('scroll', updateFades, { passive: true });
strip.addEventListener('focusin', schedule);
strip.addEventListener('focusout', schedule);
try {
  await dioxus.recv();
} finally {
  cancelAnimationFrame(frame);
  mutations.disconnect();
  releaseStrip();
  for (const release of children.values()) release();
  strip.removeEventListener('scroll', updateFades);
  strip.removeEventListener('focusin', schedule);
  strip.removeEventListener('focusout', schedule);
  strip.removeAttribute(leftFade);
  strip.removeAttribute(rightFade);
}
