const id = await dioxus.recv();
const card = document.getElementById(id);
const anchor = document.getElementById(`${id}-trigger`);
if (!card || !anchor) return;
const contains = node => anchor.contains(node) || card.contains(node);
let timer;
let hovered = false;
let lastTrigger;
const isControlled = card.dataset.hoverControlled === 'true';
const schedule = (open, delay) => {
  if (isControlled) return;
  clearTimeout(timer);
  timer = setTimeout(() => {
    if (!open && (hovered || contains(document.activeElement))) return;
    dioxus.send(open);
  }, Number(delay));
};
const enter = event => {
  if (event.pointerType !== 'mouse') return;
  hovered = true;
  schedule(true, card.dataset.hoverDelay);
};
const leave = event => {
  if (event.pointerType !== 'mouse' || contains(event.relatedTarget)) return;
  hovered = false;
  schedule(false, card.dataset.hideDelay);
};
const focus = event => {
  lastTrigger = event.target;
  if (!card.contains(event.relatedTarget) && event.target.matches(':focus-visible')) schedule(true, 0);
};
const blur = event => {
  if (!contains(event.relatedTarget)) schedule(false, card.dataset.hideDelay);
};
const toggle = event => {
  if (event.newState !== 'closed') return;
  clearTimeout(timer);
  if (card.contains(document.activeElement)) {
    // A caller can open the card without ever focusing the trigger.
    const trigger = isControlled && !anchor.contains(lastTrigger) ? anchor.firstElementChild : lastTrigger;
    trigger?.focus?.({preventScroll: true});
  }
};
for (const node of [anchor, card]) {
  node.addEventListener('pointerenter', enter);
  node.addEventListener('pointerleave', leave);
  node.addEventListener('focusout', blur);
}
anchor.addEventListener('focusin', focus);
card.addEventListener('beforetoggle', toggle);
try { await dioxus.recv(); }
finally {
  clearTimeout(timer);
  for (const node of [anchor, card]) {
    node.removeEventListener('pointerenter', enter);
    node.removeEventListener('pointerleave', leave);
    node.removeEventListener('focusout', blur);
  }
  anchor.removeEventListener('focusin', focus);
  card.removeEventListener('beforetoggle', toggle);
}
