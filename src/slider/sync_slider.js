const id = await dioxus.recv();
const input = document.getElementById(id);
if (!input) return;
const sync = () => { input.value = input.dataset.value; };
let pending;
const restore = () => {
  clearTimeout(pending);
  pending = setTimeout(sync, 0);
};
const observer = new MutationObserver(sync);
observer.observe(input, {attributes: true, attributeFilter: ['data-value', 'min', 'max', 'step']});
input.addEventListener('change', restore);
sync();
try { await dioxus.recv(); }
finally {
  clearTimeout(pending);
  observer.disconnect();
  input.removeEventListener('change', restore);
}
