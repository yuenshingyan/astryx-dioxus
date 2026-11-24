const select = document.getElementById(await dioxus.recv());
if (!select) return;
const sync = () => {
  for (const option of select.options) option.selected = option.dataset.astryxSelected === 'true';
};
let pending;
const change = () => {
  if (select.disabled) return;
  dioxus.send(Array.from(select.selectedOptions, option => option.value));
  clearTimeout(pending);
  pending = setTimeout(sync, 0);
};
const observer = new MutationObserver(sync);
observer.observe(select, {subtree: true, childList: true, attributes: true, attributeFilter: ['data-astryx-selected', 'value']});
select.addEventListener('change', change);
try { sync(); await dioxus.recv(); }
finally {
  observer.disconnect();
  clearTimeout(pending);
  select.removeEventListener('change', change);
}
