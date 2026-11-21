const [marker, initialId, interactiveSelectors] = await dioxus.recv();
const root = document.querySelector(`[data-astryx-input-description="${CSS.escape(marker)}"]`);
if (!root) return;
let inputId = initialId;
const inputTypes = new Set(['text', 'password', 'email', 'number', 'search', 'tel', 'url', 'date', 'datetime-local', 'month', 'time', 'week']);
const popupTypes = new Set(['true', 'menu', 'listbox', 'tree', 'grid', 'dialog']);
const click = event => {
  if (inputId === null || event.defaultPrevented || event.button !== 0) return;
  const target = event.target.closest?.(`${interactiveSelectors},label,[contenteditable]:not([contenteditable="false"])`);
  if (target && root.contains(target)) return;
  const doc = root.ownerDocument;
  const selection = doc.getSelection();
  if (selection && !selection.isCollapsed && root.contains(selection.anchorNode)) return;
  const control = doc.getElementById(inputId);
  if (!control || root.contains(control) || control.contains(root) || control.matches(':disabled,[aria-disabled="true"]') || control.closest('[inert]')) return;
  const isPopup = control.getAttribute('role') === 'combobox' || popupTypes.has(control.getAttribute('aria-haspopup'));
  if (!isPopup && (control.matches('textarea') || (control.matches('input') && inputTypes.has(control.type)))) control.focus();
  else control.click?.();
};
// Let any caller-owned Dioxus click handlers cancel before delegating.
root.ownerDocument.addEventListener('click', onClick);
function onClick(event) { if (root.contains(event.target)) click(event); }
try {
  while (true) {
    const next = await dioxus.recv();
    if (next === false) break;
    inputId = next;
  }
} finally {
  root.ownerDocument.removeEventListener('click', onClick);
}
