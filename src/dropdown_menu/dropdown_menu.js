const id = await dioxus.recv();
const menu = document.getElementById(id);
const trigger = document.getElementById(`${id}-trigger`);
if (!menu || !trigger || typeof menu.showPopover !== 'function') return;

const isSubMenu = menu.hasAttribute('data-dropdown-sub-menu');
const isOpen = () => menu.matches(':popover-open');
const rowSelector = '[role="menuitem"],[role="menuitemcheckbox"],[role="menuitemradio"]';
const owns = node => node?.closest?.('[role="menu"]') === menu;
const rows = () => [...menu.querySelectorAll(rowSelector)].filter(row =>
  owns(row) && !row.closest('[hidden],[inert]') && row.getClientRects().length && getComputedStyle(row).visibility === 'visible');
const open = (last = false) => {
  menu.showPopover({source: trigger});
  const items = rows();
  (items.at(last ? -1 : 0) ?? menu).focus({preventScroll: true});
};
const onToggle = event => {
  trigger.setAttribute('aria-expanded', String(event.newState === 'open'));
  if (event.newState === 'closed' && menu.contains(document.activeElement)) {
    // Restoring focus can open the trigger tooltip. Wait for native dismissal to finish.
    queueMicrotask(() => {
      const focused = document.activeElement;
      if (!isOpen() && trigger.isConnected && (focused === document.body || menu.contains(focused))) {
        trigger.focus({preventScroll: true});
      }
    });
  }
};
const rootMenu = () => {
  let root = menu;
  while (root.hasAttribute('data-dropdown-sub-menu')) {
    const parent = root.parentElement?.closest('.astryx-dropdown-menu');
    if (!parent?.matches(':popover-open')) break;
    root = parent;
  }
  return root;
};
const onClick = event => {
  if (event.defaultPrevented) return;
  if (trigger.contains(event.target)) {
    if (trigger.matches(':disabled,[aria-disabled="true"]')) return;
    if (isOpen()) menu.hidePopover();
    else open();
  } else if (isOpen() && owns(event.target)) {
    const row = event.target.closest(rowSelector);
    if (row && !row.matches(':disabled,[aria-disabled="true"],[data-dropdown-sub-menu-trigger]')) {
      rootMenu().hidePopover();
    }
  }
};
const onKey = event => {
  if (event.defaultPrevented || event.isComposing || event.altKey || event.ctrlKey || event.metaKey) return;
  const rtl = getComputedStyle(trigger).direction === 'rtl';
  const openKey = rtl ? 'ArrowLeft' : 'ArrowRight';
  const closeKey = rtl ? 'ArrowRight' : 'ArrowLeft';
  const opens = isSubMenu ? event.key === openKey : ['ArrowDown', 'ArrowUp'].includes(event.key);
  if (event.target === trigger && !trigger.matches(':disabled,[aria-disabled="true"]') && opens) {
    event.preventDefault();
    if (!isOpen()) open(event.key === 'ArrowUp');
    return;
  }
  if (!isOpen() || !owns(event.target)) return;
  if (isSubMenu && (event.key === closeKey || event.key === 'Escape')) {
    event.preventDefault();
    menu.hidePopover();
    return;
  }
  if (event.key === 'Tab') {
    const root = rootMenu();
    document.getElementById(`${root.id}-trigger`)?.focus({preventScroll: true});
    root.hidePopover();
    return;
  }
  const items = rows();
  if (!items.length) return;
  const index = items.indexOf(event.target);
  let next;
  switch (event.key) {
    case 'ArrowDown': next = (index + 1) % items.length; break;
    case 'ArrowUp': next = index < 0 ? items.length - 1 : (index + items.length - 1) % items.length; break;
    case 'Home': next = 0; break;
    case 'End': next = items.length - 1; break;
    case 'Enter': case ' ':
      if (index < 0) return;
      event.preventDefault();
      if (!items[index].matches(':disabled,[aria-disabled="true"]')) items[index].click();
      return;
    default: {
      if ([...event.key].length !== 1) return;
      const letter = event.key.toLocaleLowerCase();
      for (let offset = 1; offset <= items.length; offset++) {
        const candidate = (index + offset) % items.length;
        const name = items[candidate].getAttribute('aria-label') ?? items[candidate].textContent;
        if (name.trim().toLocaleLowerCase().startsWith(letter)) { next = candidate; break; }
      }
      if (next === undefined) return;
    }
  }
  event.preventDefault();
  items[next].focus({preventScroll: true});
  items[next].scrollIntoView({block: 'nearest', inline: 'nearest'});
};
menu.addEventListener('beforetoggle', onToggle);
document.addEventListener('click', onClick);
document.addEventListener('keydown', onKey);
try { await dioxus.recv(); }
finally {
  menu.removeEventListener('beforetoggle', onToggle);
  document.removeEventListener('click', onClick);
  document.removeEventListener('keydown', onKey);
}
