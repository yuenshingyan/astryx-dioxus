const root = document.getElementById(await dioxus.recv());
if (!root) return;
const itemSelector = '[role="treeitem"]';
const owned = item => item?.closest('[role="tree"]') === root;
const visible = () => [...root.querySelectorAll(itemSelector)].filter(item =>
  owned(item) && !item.closest('[hidden],[inert]') && item.getClientRects().length);
let current;
const typeahead = createTreeTypeahead();
const sync = () => {
  const items = visible();
  if (!items.includes(current)) current = items[0];
  for (const item of root.querySelectorAll(itemSelector)) {
    if (!owned(item)) continue;
    const index = item === current ? 0 : -1;
    if (item.tabIndex !== index) item.tabIndex = index;
  }
};
const focus = event => {
  if (event.target.matches(itemSelector) && owned(event.target)) {
    current = event.target;
    sync();
  }
};
const click = event => {
  const row = event.target.closest('.astryx-tree-list__row');
  const item = row?.parentElement;
  if (owned(item)) item.focus({preventScroll: true});
};
const key = event => {
  const item = event.target;
  if (event.defaultPrevented || event.isComposing || event.ctrlKey || event.altKey || event.metaKey ||
      !item.matches?.(itemSelector) || !owned(item)) return;
  const items = visible();
  const index = items.indexOf(item);
  const rtl = getComputedStyle(root).direction === 'rtl';
  const activate = () => item.querySelector(':scope > .astryx-tree-list__row')?.click();
  let next;
  switch (event.key) {
    case 'ArrowDown': next = items[index + 1]; break;
    case 'ArrowUp': next = items[index - 1]; break;
    case 'Home': next = items[0]; break;
    case 'End': next = items.at(-1); break;
    case rtl ? 'ArrowLeft' : 'ArrowRight':
      if (item.getAttribute('aria-expanded') === 'false') activate();
      else if (item.getAttribute('aria-expanded') === 'true') next = item.querySelector(':scope > [role="group"] > [role="treeitem"]');
      break;
    case rtl ? 'ArrowRight' : 'ArrowLeft':
      if (item.getAttribute('aria-expanded') === 'true') activate();
      else next = item.parentElement.closest(itemSelector);
      break;
    case 'Enter': case ' ': activate(); break;
    default:
      next = typeahead(event, items, index);
      if (!next) return;
  }
  event.preventDefault();
  if (owned(next)) next.focus();
};
const observer = new MutationObserver(sync);
observer.observe(root, {childList: true, subtree: true, attributes: true, attributeFilter: ['hidden', 'tabindex']});
root.addEventListener('focusin', focus);
root.addEventListener('click', click);
document.addEventListener('keydown', key);
try { sync(); await dioxus.recv(); }
finally {
  observer.disconnect();
  root.removeEventListener('focusin', focus);
  root.removeEventListener('click', click);
  document.removeEventListener('keydown', key);
}
