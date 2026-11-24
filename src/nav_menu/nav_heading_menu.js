const owner = await dioxus.recv();
const root = document.querySelector(`[data-astryx-nav-menu-owner="${owner}"]`);
if (!root) return;
const onKey = event => {
  if (event.defaultPrevented || event.isComposing || event.ctrlKey || event.altKey || event.metaKey) return;
  if (event.target.closest?.('[role="menu"]') !== root) return;
  const rows = [...root.querySelectorAll('[role="menuitem"], [role="menuitemradio"]')].filter(row =>
    row.closest('[role="menu"]') === root && !row.closest('[hidden],[inert]') && row.getClientRects().length && getComputedStyle(row).visibility === 'visible');
  if (!rows.length) return;
  const index = rows.indexOf(event.target);
  if (index < 0 && event.target !== root) return;
  let next;
  switch (event.key) {
    case 'ArrowDown': next = (index + 1) % rows.length; break;
    case 'ArrowUp': next = index < 0 ? rows.length - 1 : (index + rows.length - 1) % rows.length; break;
    case 'Home': next = 0; break;
    case 'End': next = rows.length - 1; break;
    case 'Enter': case ' ':
      if (index < 0) return;
      event.preventDefault();
      if (!rows[index].matches(':disabled,[aria-disabled="true"]')) rows[index].click();
      return;
    default: {
      if ([...event.key].length !== 1) return;
      const key = event.key.toLocaleLowerCase();
      for (let offset = 1; offset <= rows.length; offset++) {
        const candidate = (index + offset) % rows.length;
        const label = rows[candidate].getAttribute('aria-label') ?? rows[candidate].textContent;
        if (label.trim().toLocaleLowerCase().startsWith(key)) { next = candidate; break; }
      }
      if (next === undefined) return;
    }
  }
  event.preventDefault();
  rows[next].focus();
};
// Run after the renderer's delegated handlers so caller cancellation wins.
document.addEventListener('keydown', onKey);
try { await dioxus.recv(); }
finally { document.removeEventListener('keydown', onKey); }
