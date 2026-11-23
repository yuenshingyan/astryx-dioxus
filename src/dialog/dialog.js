const id = await dioxus.recv();
const dialog = document.querySelector(`[data-astryx-dialog-owner="${CSS.escape(id)}"]`);
if (!dialog) return;
const sync = () => {
  if (dialog.dataset.open === 'true' && dialog.isConnected && !dialog.open) {
    dialog.showModal();
    const target = [...dialog.querySelectorAll('[data-autofocus]')].find(node => node.closest('dialog') === dialog);
    target?.focus({preventScroll: true});
  } else if (dialog.dataset.open !== 'true' && dialog.open) dialog.close();
};
const cancel = event => {
  event.preventDefault();
  if (dialog.dataset.purpose !== 'required') dioxus.send(true);
};
const outside = event => {
  if (event.target !== dialog) return false;
  const {left, right, top, bottom} = dialog.getBoundingClientRect();
  return event.clientX < left || event.clientX >= right || event.clientY < top || event.clientY >= bottom;
};
let beganOutside = false;
const down = event => { beganOutside = event.button === 0 && outside(event); };
const click = event => {
  if (beganOutside && outside(event) && dialog.dataset.purpose === 'info') dioxus.send(true);
  beganOutside = false;
};
const observer = new MutationObserver(sync);
observer.observe(dialog, {attributes: true, attributeFilter: ['data-open']});
dialog.addEventListener('cancel', cancel);
dialog.addEventListener('pointerdown', down);
dialog.addEventListener('click', click);
try {
  sync();
  await dioxus.recv();
} finally {
  observer.disconnect();
  dialog.removeEventListener('cancel', cancel);
  dialog.removeEventListener('pointerdown', down);
  dialog.removeEventListener('click', click);
  if (dialog.open) dialog.close();
}
