// Adapted from useScrollRegionProps and observeMainOverflow: measure only
// scroll-capable axes, retain the last result while hidden, and coalesce work.
const [marker, id] = await dioxus.recv();
const root = document.querySelector(`[${CSS.escape(marker)}="${CSS.escape(id)}"]`);
if (!root) return;
let frame;
let previous;
let active = true;
const measure = () => {
  frame = undefined;
  if (!root.isConnected || !root.getClientRects().length || !root.clientWidth || !root.clientHeight) return;
  const style = getComputedStyle(root);
  const canScroll = value => /^(auto|scroll|overlay)$/.test(value);
  const overflows = (canScroll(style.overflowX) && root.scrollWidth - root.clientWidth > 1)
    || (canScroll(style.overflowY) && root.scrollHeight - root.clientHeight > 1);
  if (previous !== overflows) { previous = overflows; dioxus.send(overflows); }
};
const schedule = () => { if (active && frame === undefined) frame = requestAnimationFrame(measure); };
const children = new Map();
const sync = () => {
  for (const [child, stop] of children) {
    if (child.parentNode !== root) { stop(); children.delete(child); }
  }
  for (const child of root.children) {
    if (!children.has(child)) children.set(child, observeResize(child, schedule));
  }
  schedule();
};
const stop = observeResize(root, schedule);
const observer = new MutationObserver(sync);
observer.observe(root, {subtree:true, childList:true, characterData:true, attributes:true, attributeFilter:['class','dir','hidden','style']});
const listeners = new AbortController();
for (const [target, event] of [[window,'resize'],[visualViewport,'resize'],[root,'load'],[root,'transitionend'],[root,'animationend'],[document.fonts,'loadingdone']]) {
  target?.addEventListener(event, schedule, {capture:true, signal:listeners.signal});
}
document.fonts?.ready.then(schedule);
sync();
try { await dioxus.recv(); }
finally {
  active = false;
  if (frame !== undefined) cancelAnimationFrame(frame);
  observer.disconnect();
  listeners.abort();
  stop();
  for (const unobserve of children.values()) unobserve();
}
