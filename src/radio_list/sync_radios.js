const owner = await dioxus.recv();
const group = document.querySelector(`[data-astryx-radio-list="${CSS.escape(owner)}"]`);
if (!group) return;
const owns = input => input.matches?.('input[data-radio-checked]') && input.closest('[role="radiogroup"]') === group;
const sync = () => {
  for (const radio of group.querySelectorAll('input[data-radio-checked]')) {
    if (owns(radio)) radio.checked = radio.dataset.radioChecked === 'true';
  }
};
let pending;
const onChange = event => {
  if (!owns(event.target)) return;
  clearTimeout(pending);
  pending = setTimeout(sync, 0);
};
const observer = new MutationObserver(sync);
observer.observe(group, {subtree: true, childList: true, attributes: true, attributeFilter: ['data-radio-checked']});
group.addEventListener('change', onChange);
sync();
try { await dioxus.recv(); }
finally {
  clearTimeout(pending);
  observer.disconnect();
  group.removeEventListener('change', onChange);
}
