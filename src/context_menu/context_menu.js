const id = await dioxus.recv();
const menu = document.getElementById(id);
const trigger = document.getElementById(`${id}-trigger`);
if (!menu || !trigger || typeof menu.showPopover !== 'function') return;

const selector = '[role="menuitem"], [role="menuitemradio"], [role="menuitemcheckbox"]';
const isOpen = () => menu.matches(':popover-open');
const isDisabled = () => trigger.dataset.contextMenuDisabled === 'true';
let returnFocus;
const open = (x, y, origin) => {
  returnFocus = origin instanceof HTMLElement ? origin : trigger;
  menu.style.left = `${x}px`;
  menu.style.top = `${y}px`;
  if (!isOpen()) menu.showPopover({source: trigger});
  const rect = menu.getBoundingClientRect();
  menu.style.left = `${Math.max(0, Math.min(x, document.documentElement.clientWidth - rect.width))}px`;
  menu.style.top = `${Math.max(0, Math.min(y, document.documentElement.clientHeight - rect.height))}px`;
  const first = [...menu.querySelectorAll(selector)].find(row =>
    row.closest('[role="menu"]') === menu && !row.closest('[hidden],[inert]') && row.getClientRects().length);
  (first ?? menu).focus({preventScroll: true});
};
const context = event => {
  if (event.defaultPrevented || isDisabled() || event.pointerType === 'touch') return;
  event.preventDefault();
  const origin = trigger.contains(document.activeElement) ? document.activeElement : trigger;
  open(event.clientX, event.clientY, origin);
};
const key = event => {
  if (event.defaultPrevented || event.isComposing) return;
  if (!isDisabled() && trigger.contains(event.target) &&
      (event.key === 'ContextMenu' || (event.shiftKey && event.key === 'F10'))) {
    event.preventDefault();
    const rect = event.target.getBoundingClientRect();
    open(rect.left, rect.bottom, event.target);
  } else if (isOpen() && menu.contains(event.target) && event.key === 'Tab') {
    returnFocus?.focus({preventScroll: true});
    menu.hidePopover();
  }
};
const select = event => {
  if (!isOpen() || event.defaultPrevented) return;
  const row = event.target.closest?.(selector);
  if (row?.closest('[role="menu"]') === menu && !row.matches('[aria-disabled="true"],:disabled')) {
    returnFocus?.focus({preventScroll: true});
    menu.hidePopover();
  }
};
const beforeToggle = event => {
  if (event.newState === 'closed' && menu.contains(document.activeElement)) returnFocus?.focus({preventScroll: true});
};
const toggle = event => dioxus.send(event.newState === 'open');
const observer = new MutationObserver(() => { if (isDisabled() && isOpen()) menu.hidePopover(); });
observer.observe(trigger, {attributes: true, attributeFilter: ['data-context-menu-disabled']});
trigger.addEventListener('contextmenu', context);
menu.addEventListener('beforetoggle', beforeToggle);
menu.addEventListener('toggle', toggle);
document.addEventListener('keydown', key);
document.addEventListener('click', select);
try { await dioxus.recv(); }
finally {
  observer.disconnect();
  trigger.removeEventListener('contextmenu', context);
  menu.removeEventListener('beforetoggle', beforeToggle);
  menu.removeEventListener('toggle', toggle);
  document.removeEventListener('keydown', key);
  document.removeEventListener('click', select);
  if (isOpen()) menu.hidePopover();
}
