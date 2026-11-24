const id = await dioxus.recv();
const menu = document.getElementById(id);
const trigger = document.getElementById(`${id}-trigger`);
if (!menu || !trigger) return;
const onToggle = () => {
  if (!menu.matches(':popover-open')) return;
  (menu.querySelector('[role="menuitem"]:not([aria-disabled="true"])') ?? menu).focus({preventScroll: true});
};
const onTab = event => {
  if (event.defaultPrevented || event.key !== 'Tab' || !menu.contains(event.target)) return;
  trigger.focus({preventScroll: true});
  dioxus.send(true);
};
menu.addEventListener('toggle', onToggle);
document.addEventListener('keydown', onTab);
try { await dioxus.recv(); }
finally {
  menu.removeEventListener('toggle', onToggle);
  document.removeEventListener('keydown', onTab);
}
