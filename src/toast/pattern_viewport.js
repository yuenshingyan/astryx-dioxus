// Private bottomEnd/info viewport adapter, owned by ToastViewport.
const id = await dioxus.recv();
const viewport = document.getElementById(id);
if (!viewport) return;
const announcer = installAnnounce().createAnnouncer();
// Let assistive technology discover the empty live region before the first action.
announcer.prepare(viewport);
const announced = new Set();
let previous = document.activeElement;
let returnTarget = previous;
const firstControl = row => row?.querySelector('button:not(:disabled)');
const rows = () => [...viewport.querySelectorAll('.astryx-demo-toast-row:not([data-exiting])')];
const onFocus = event => {
  if (!viewport.contains(event.target)) previous = event.target;
  else if (!viewport.contains(event.relatedTarget)) returnTarget = previous;
};
const onKey = event => {
  if (event.key !== 'F6' || !rows().length) return;
  const target = firstControl(rows().at(-1));
  if (target) { event.preventDefault(); target.focus(); }
};
const sync = () => {
  if (viewport.children.length) {
    if (!viewport.matches(':popover-open')) viewport.showPopover();
  } else if (viewport.matches(':popover-open')) viewport.hidePopover();
};
viewport.astryxDemoToast = {
  announce(row, text) {
    const key = row.dataset.toastId;
    if (announced.has(key)) return;
    announced.add(key);
    announcer.announce(row, text, () => row.isConnected && !row.hasAttribute('data-exiting'));
  },
  handoff(row) {
    if (!row.contains(document.activeElement)) return;
    const target = rows().filter(e => e !== row).map(firstControl).at(-1);
    if (target) target.focus();
    else if (returnTarget?.isConnected) returnTarget.focus();
  },
};
const observer = new MutationObserver(sync);
observer.observe(viewport, {childList:true});
document.addEventListener('focusin', onFocus, true);
document.addEventListener('keydown', onKey);
sync();
try { await dioxus.recv(); }
finally {
  observer.disconnect();
  document.removeEventListener('focusin', onFocus, true);
  document.removeEventListener('keydown', onKey);
  announcer.dispose();
  if (viewport.matches(':popover-open')) viewport.hidePopover();
  delete viewport.astryxDemoToast;
}
