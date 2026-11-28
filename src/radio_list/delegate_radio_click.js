const [id, interactiveSelectors] = await dioxus.recv();
const radio = document.getElementById(id);
const row = radio?.closest('.astryx-radio-list-item');
if (!row) return;

const nestedControl = `${interactiveSelectors},summary,label,[contenteditable]:not([contenteditable="false"]),[tabindex]`;
const delegate = event => {
  if (radio.disabled || event.defaultPrevented || event.button !== 0) return;
  const selection = row.ownerDocument.getSelection();
  if (selection && !selection.isCollapsed && row.contains(selection.anchorNode)) return;
  for (const node of event.composedPath()) {
    if (node === row) break;
    // The input and any other control own their clicks, even from inside rich slots.
    if (node.matches?.(nestedControl)) return;
  }
  // Bubble exactly one click: the radio's native activation, not the row click.
  event.stopPropagation();
  radio.focus({preventScroll: true});
  radio.click();
};
row.addEventListener('click', delegate);
try { await dioxus.recv(); }
finally { row.removeEventListener('click', delegate); }
