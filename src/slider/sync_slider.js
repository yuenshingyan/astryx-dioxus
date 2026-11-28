const id = await dioxus.recv();
const input = document.getElementById(id);
if (!input) return;
const sync = () => { input.value = input.dataset.value; };
let pending;
const restore = () => {
  clearTimeout(pending);
  pending = setTimeout(sync, 0);
};
const onPageKey = event => {
  if (input.disabled || event.defaultPrevented || (event.key !== 'PageUp' && event.key !== 'PageDown')) return;
  event.preventDefault();
  const previous = input.value;
  // Native stepping keeps the range's step grid and form value in sync.
  if (event.key === 'PageUp') input.stepUp(10);
  else input.stepDown(10);
  if (input.value === previous) return;
  // Programmatic stepping is silent; use the existing callbacks and rollback path.
  input.dispatchEvent(new Event('input', {bubbles: true, composed: true}));
  input.dispatchEvent(new Event('change', {bubbles: true}));
};
const observer = new MutationObserver(sync);
observer.observe(input, {attributes: true, attributeFilter: ['data-value', 'min', 'max', 'step']});
input.addEventListener('change', restore);
input.addEventListener('keydown', onPageKey);
sync();
try { await dioxus.recv(); }
finally {
  clearTimeout(pending);
  observer.disconnect();
  input.removeEventListener('change', restore);
  input.removeEventListener('keydown', onPageKey);
}
