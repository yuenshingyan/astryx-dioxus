const owner = await dioxus.recv();
const isTabStrip = Array.isArray(owner);
const id = isTabStrip ? owner[0] : owner;
const marker = isTabStrip ? 'data-astryx-tab-list-owner' : 'data-astryx-toolbar-owner';
const root = document.querySelector(`[${marker}="${CSS.escape(id)}"]`);
if (!root) return;
const boundary = '[role="toolbar"], [popover], [role="tablist"], [role="radiogroup"], [role="menu"], [data-astryx-tab-list-owner]';
const selector = isTabStrip ? '.astryx-tab[data-tab-value]' : 'button,input,a[href],[tabindex]';
const selected = item => isTabStrip && item.matches('[aria-current="true"], [aria-selected="true"]');
const saved = new Map();
let current;
const restore = (item, value) => value == null ? item.removeAttribute('tabindex') : item.setAttribute('tabindex', value);
const collect = () => [...root.querySelectorAll(selector)].filter(item =>
  item.closest(boundary) === root && !item.matches(':disabled') && !item.closest('[hidden],[inert]') &&
  item.getClientRects().length > 0 && getComputedStyle(item).visibility === 'visible');
const disabled = item => item.matches('[aria-disabled="true"]');
const sync = (preferred) => {
  const items = collect();
  for (const [item, value] of saved) {
    if (!items.includes(item)) {
      restore(item, value);
      saved.delete(item);
    }
  }
  current = items.includes(preferred) ? preferred : items.includes(current) && !disabled(current) ? current : items.find(selected) ?? items.find(item => !disabled(item)) ?? items[0];
  for (const item of items) {
    if (!saved.has(item)) saved.set(item, item.getAttribute('tabindex'));
    const nextTabIndex = item === current ? 0 : -1;
    if (item.getAttribute('tabindex') !== String(nextTabIndex)) item.tabIndex = nextTabIndex;
  }
  return items;
};
const onFocus = event => {
  if (event.target.closest?.(boundary) === root && collect().includes(event.target)) sync(event.target);
};
const onKey = event => {
  if (event.defaultPrevented || event.isComposing || event.shiftKey || event.ctrlKey || event.altKey || event.metaKey || event.target.closest?.(boundary) !== root) return;
  if (event.target.matches('textarea,input:not([type="button"]):not([type="submit"]):not([type="reset"]),[contenteditable]:not([contenteditable="false"])')) return;
  const vertical = root.getAttribute('aria-orientation') === 'vertical';
  const rtl = getComputedStyle(root).direction === 'rtl';
  const forward = vertical ? 'ArrowDown' : rtl ? 'ArrowLeft' : 'ArrowRight';
  const backward = vertical ? 'ArrowUp' : rtl ? 'ArrowRight' : 'ArrowLeft';
  const bothAxes = isTabStrip && root.getAttribute('role') !== 'tablist';
  const goesForward = event.key === forward || (bothAxes && event.key === 'ArrowDown');
  const goesBackward = event.key === backward || (bothAxes && event.key === 'ArrowUp');
  if (!goesForward && !goesBackward && event.key !== 'Home' && event.key !== 'End') return;
  if (isTabStrip && !collect().includes(event.target)) return;
  const items = collect().filter(item => !disabled(item));
  if (!items.length) return;
  const index = items.indexOf(event.target);
  const next = event.key === 'Home' ? 0 : event.key === 'End' ? items.length - 1 :
    goesForward ? (index + 1) % items.length : index <= 0 ? items.length - 1 : index - 1;
  event.preventDefault();
  items[next].focus();
};
const observer = new MutationObserver(() => sync(document.activeElement));
const attributeFilter = ['disabled', 'aria-disabled', 'hidden', 'inert', 'style', 'class'];
if (isTabStrip) attributeFilter.push('aria-selected', 'aria-current', 'tabindex');
observer.observe(root, {childList: true, subtree: true, attributes: true, attributeFilter});
sync(document.activeElement);
root.addEventListener('focusin', onFocus);
document.addEventListener('keydown', onKey);
try { await dioxus.recv(); }
finally {
  observer.disconnect();
  root.removeEventListener('focusin', onFocus);
  document.removeEventListener('keydown', onKey);
  for (const [item, value] of saved) restore(item, value);
}
