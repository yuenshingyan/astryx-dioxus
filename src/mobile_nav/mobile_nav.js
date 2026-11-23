const id = await dioxus.recv();
const dialog = document.querySelector(`[data-astryx-mobile-nav-owner="${CSS.escape(id)}"]`);
if (!dialog) return;
const sync = () => {
  if (dialog.dataset.open === 'true' && dialog.isConnected && !dialog.open) dialog.showModal();
  else if (dialog.dataset.open !== 'true' && dialog.open) dialog.close();
};
const onCancel = event => {
  event.preventDefault();
  dioxus.send(true);
};
let beganOutside = false;
const onDown = event => { beganOutside = event.target === dialog; };
const onClick = event => {
  if (beganOutside && event.target === dialog) dioxus.send(true);
  beganOutside = false;
};
const observer = new MutationObserver(sync);
observer.observe(dialog, {attributes: true, attributeFilter: ['data-open']});
dialog.addEventListener('cancel', onCancel);
dialog.addEventListener('pointerdown', onDown);
dialog.addEventListener('click', onClick);
try {
  sync();
  await dioxus.recv();
} finally {
  observer.disconnect();
  dialog.removeEventListener('cancel', onCancel);
  dialog.removeEventListener('pointerdown', onDown);
  dialog.removeEventListener('click', onClick);
  if (dialog.open) dialog.close();
}
