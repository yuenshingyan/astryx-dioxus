const id = await dioxus.recv();
const input = document.querySelector(`[data-astryx-checkbox-owner="${CSS.escape(id)}"]`);
if (!input) return;
const sync = () => {
  input.checked = input.dataset.checked === 'true';
  input.indeterminate = input.dataset.mixed === 'true';
};
// A native change can mutate the control even if the caller keeps its old value.
let pending;
const restore = () => {
  clearTimeout(pending);
  pending = setTimeout(sync, 0);
};
const observer = new MutationObserver(sync);
observer.observe(input, {attributes: true, attributeFilter: ['data-checked', 'data-mixed']});
input.addEventListener('change', restore);
sync();
try { await dioxus.recv(); }
finally {
  clearTimeout(pending);
  observer.disconnect();
  input.removeEventListener('change', restore);
}
