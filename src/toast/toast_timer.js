const toast = document.getElementById(await dioxus.recv());
if (!toast) return;
let timer;
let started;
let remaining;
let hovered = toast.matches(':hover');
const stop = () => {
  if (timer === undefined) return;
  clearTimeout(timer);
  timer = undefined;
  remaining = Math.max(0, remaining - (performance.now() - started));
};
const sync = () => {
  if (hovered || toast.contains(document.activeElement)) {
    stop();
  } else if (remaining !== undefined && timer === undefined) {
    started = performance.now();
    timer = setTimeout(() => {
      timer = undefined;
      remaining = undefined;
      dioxus.send(true);
    }, remaining);
  }
};
const reset = () => {
  stop();
  remaining = toast.hasAttribute('data-duration') ? Number(toast.dataset.duration) : undefined;
  sync();
};
const enter = () => { hovered = true; sync(); };
const leave = () => { hovered = false; sync(); };
const focusOut = () => queueMicrotask(() => { if (toast.isConnected) sync(); });
const observer = new MutationObserver(reset);
observer.observe(toast, {attributes: true, attributeFilter: ['data-duration']});
toast.addEventListener('mouseenter', enter);
toast.addEventListener('mouseleave', leave);
toast.addEventListener('focusin', sync);
toast.addEventListener('focusout', focusOut);
try { reset(); await dioxus.recv(); }
finally {
  observer.disconnect();
  stop();
  remaining = undefined;
  toast.removeEventListener('mouseenter', enter);
  toast.removeEventListener('mouseleave', leave);
  toast.removeEventListener('focusin', sync);
  toast.removeEventListener('focusout', focusOut);
}
