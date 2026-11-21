const owner = await dioxus.recv();
const root = document.querySelector(`[data-astryx-avatar-group="${owner}"]`);
if (!root) return;
const hint = root.querySelector(':scope > [data-astryx-avatar-group-hint]');
const originalTabs = new Map();
let active;
const items = () => [...root.querySelectorAll(`[data-avatar-item][data-astryx-avatar-group-member="${owner}"]`)]
  .filter(item => !item.matches(':disabled,[aria-disabled="true"]') && !item.closest('[hidden],[inert]'));
const describe = enabled => {
  const current = root.getAttribute('aria-describedby');
  const ids = (current ?? '').split(/\s+/).filter(id => id && id !== hint.id);
  if (enabled) ids.push(hint.id);
  const next = ids.join(' ') || null;
  if (next !== current) {
    if (next) root.setAttribute('aria-describedby', next);
    else root.removeAttribute('aria-describedby');
  }
  if (hint.hidden === enabled) hint.hidden = !enabled;
};
const restore = (item, tab) => {
  if (tab === null) item.removeAttribute('tabindex');
  else item.setAttribute('tabindex', tab);
};
const sync = () => {
  const members = items();
  for (const [item, tab] of originalTabs) {
    if (!members.includes(item)) {
      restore(item, tab);
      originalTabs.delete(item);
    }
  }
  active = members.includes(active) ? active : members[0];
  for (const item of members) {
    if (!originalTabs.has(item)) originalTabs.set(item, item.getAttribute('tabindex'));
    const index = item === active ? 0 : -1;
    if (item.getAttribute('tabindex') !== String(index)) item.tabIndex = index;
  }
  describe(members.length > 0);
};
const onFocus = event => {
  if (!event.defaultPrevented && items().includes(event.target)) {
    active = event.target;
    sync();
  }
};
const onKey = event => {
  if (event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;
  const members = items();
  const index = members.indexOf(event.target);
  if (index < 0) return;
  const isRtl = getComputedStyle(root).direction === 'rtl';
  let next;
  switch (event.key) {
    case 'Home': next = 0; break;
    case 'End': next = members.length - 1; break;
    case 'ArrowLeft': next = index + (isRtl ? 1 : -1); break;
    case 'ArrowRight': next = index + (isRtl ? -1 : 1); break;
    default: return;
  }
  event.preventDefault();
  active = members[(next + members.length) % members.length];
  sync();
  active.focus();
};
const observer = new MutationObserver(sync);
observer.observe(root, {childList: true, subtree: true, attributes: true,
  attributeFilter: ['data-avatar-item', 'data-astryx-avatar-group-member', 'aria-describedby', 'aria-disabled', 'disabled', 'hidden', 'inert', 'tabindex']});
// Bubble after Dioxus's delegated handlers so caller cancellation wins.
document.addEventListener('focusin', onFocus);
document.addEventListener('keydown', onKey);
try {
  sync();
  await dioxus.recv();
} finally {
  observer.disconnect();
  document.removeEventListener('focusin', onFocus);
  document.removeEventListener('keydown', onKey);
  for (const [item, tab] of originalTabs) restore(item, tab);
  describe(false);
}
