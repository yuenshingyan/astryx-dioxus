const id = await dioxus.recv();
const dialog = document.querySelector(`[data-astryx-mobile-nav-owner="${CSS.escape(id)}"]`);
if (!dialog) return;
const ownerDocument = dialog.ownerDocument;
let opener = null;
const closeAndRestoreFocus = () => {
  const target = opener;
  opener = null;
  // The opener is inert until close(), and the native restore can miss it.
  dialog.close();
  if (target?.isConnected && target !== ownerDocument.activeElement) target.focus?.();
};
const sync = () => {
  if (dialog.dataset.open === 'true' && dialog.isConnected && !dialog.open) {
    const active = ownerDocument.activeElement;
    opener = active && active !== ownerDocument.body && !dialog.contains(active) ? active : null;
    dialog.showModal();
  } else if (dialog.dataset.open !== 'true' && dialog.open) closeAndRestoreFocus();
};
const onCancel = event => {
  // A descendant can cancel its own picker without dismissing navigation.
  if (event.target !== dialog) return;
  event.preventDefault();
  if (dialog.open && dialog.dataset.open === 'true') dioxus.send(true);
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
  if (dialog.open) closeAndRestoreFocus();
}
