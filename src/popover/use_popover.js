const id = await dioxus.recv();
const surface = document.getElementById(id);
const anchor = document.getElementById(`${id}-trigger`);
if (!surface || !anchor || typeof surface.showPopover !== 'function') return;
const name = anchor.style.getPropertyValue('anchor-name');
const priority = anchor.style.getPropertyPriority('anchor-name');
anchor.style.setProperty('anchor-name', `--${id}`);
const sync = () => {
  const isOpen = surface.matches(':popover-open');
  if (surface.dataset.open === 'true' && !isOpen) {
    surface.showPopover();
    if (surface.dataset.popoverMenuFocus === 'true') {
      (surface.querySelector('[role="menuitemradio"]:not([aria-disabled="true"])') ?? surface).focus({preventScroll: true});
    }
  }
  if (surface.dataset.open !== 'true' && isOpen) {
    const restore = surface.contains(document.activeElement);
    surface.hidePopover();
    if (restore && anchor.isConnected) anchor.focus({preventScroll: true});
  }
};
const owns = target => anchor.contains(target) || surface.contains(target);
const onKey = event => {
  if (!event.defaultPrevented && event.key === 'Tab' && surface.dataset.popoverMenuFocus === 'true' &&
      surface.matches(':popover-open') && surface.contains(event.target)) {
    anchor.focus({preventScroll: true});
    dioxus.send(true);
    return;
  }
  if (!event.defaultPrevented && !event.isComposing && event.key === 'Escape' &&
      surface.matches(':popover-open') && owns(event.target)) {
    event.preventDefault();
    dioxus.send(true);
  }
};
const onClick = event => {
  if (surface.matches(':popover-open') && !owns(event.target)) dioxus.send(true);
};
const observer = new MutationObserver(sync);
observer.observe(surface, {attributes: true, attributeFilter: ['data-open']});
document.addEventListener('keydown', onKey);
document.addEventListener('click', onClick);
try {
  sync();
  await dioxus.recv();
} finally {
  observer.disconnect();
  document.removeEventListener('keydown', onKey);
  document.removeEventListener('click', onClick);
  if (surface.matches(':popover-open')) surface.hidePopover();
  anchor.style.setProperty('anchor-name', name, priority);
}
