const select = document.getElementById(await dioxus.recv());
if (!select) return;
const sync = () => {
  const value = select.dataset.astryxSelectorValue;
  if (select.value !== value) select.value = value;
};
let pending;
const reconcile = () => {
  clearTimeout(pending);
  pending = setTimeout(sync, 0);
};
const observer = new MutationObserver(sync);
observer.observe(select, {attributes: true, attributeFilter: ['data-astryx-selector-value', 'value', 'selected'], subtree: true, childList: true});
select.addEventListener('change', reconcile);
try { sync(); await dioxus.recv(); }
finally { observer.disconnect(); clearTimeout(pending); select.removeEventListener('change', reconcile); }
