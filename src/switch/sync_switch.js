const owner = await dioxus.recv();
const input = document.querySelector(`[data-astryx-switch-owner="${CSS.escape(owner)}"]`);
if (!input) return;
const sync = () => { input.checked = input.dataset.checked === 'true'; };
let pending;
const restore = () => {
  clearTimeout(pending);
  pending = setTimeout(sync, 0);
};
const observer = new MutationObserver(sync);
observer.observe(input, {attributes: true, attributeFilter: ['data-checked']});
input.addEventListener('change', restore);
sync();
try { await dioxus.recv(); }
finally {
  clearTimeout(pending);
  observer.disconnect();
  input.removeEventListener('change', restore);
}
