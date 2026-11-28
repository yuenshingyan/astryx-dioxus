const root = document.getElementById(await dioxus.recv());
if (!root) return;
const items = () => [...root.querySelectorAll('.astryx-segmented-control__item')].filter(item =>
  item.closest('[role=radiogroup]') === root && !item.closest('[hidden],[inert]'));
const enabled = item => !item.matches('[aria-disabled=true],:disabled');
const sync = () => {
  const options = items();
  const stop = options.find(item => item.matches('[aria-checked=true]')) ?? options.find(enabled);
  for (const item of options) {
    const index = item === stop ? 0 : -1;
    if (item.tabIndex !== index) item.tabIndex = index;
  }
};
const navigate = event => {
  if (event.defaultPrevented || event.isComposing || event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;
  const options = items();
  if (!options.includes(event.target)) return;
  const available = options.filter(enabled);
  if (!available.length) return;
  const index = available.indexOf(event.target);
  const rtl = getComputedStyle(root).direction === 'rtl';
  let next;
  switch (event.key) {
    case 'Home': next = available[0]; break;
    case 'End': next = available.at(-1); break;
    case rtl ? 'ArrowLeft' : 'ArrowRight':
      next = available[(index + 1) % available.length]; break;
    case rtl ? 'ArrowRight' : 'ArrowLeft':
      next = available[index <= 0 ? available.length - 1 : index - 1]; break;
    default: return;
  }
  event.preventDefault();
  next.focus();
  next.click();
};
const observer = new MutationObserver(sync);
observer.observe(root, {subtree: true, childList: true, attributes: true, attributeFilter: ['aria-checked', 'aria-disabled', 'hidden', 'inert', 'tabindex']});
document.addEventListener('keydown', navigate);
try { sync(); await dioxus.recv(); }
finally { observer.disconnect(); document.removeEventListener('keydown', navigate); }
