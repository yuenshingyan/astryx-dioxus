const owner = await dioxus.recv();
const group = document.querySelector(`[data-astryx-radio-list="${CSS.escape(owner)}"]`);
if (!group) return;
const form = group.closest('form');
const controlledRadio = 'input[type="radio"][data-radio-checked]';
const owns = input => input.matches?.(controlledRadio) && input.closest('[role="radiogroup"]') === group;
const sync = () => {
  for (const radio of group.querySelectorAll(controlledRadio)) {
    if (!owns(radio)) continue;
    const checked = radio.dataset.radioChecked === 'true';
    if (radio.checked !== checked) radio.checked = checked;
  }
};
let pending;
const scheduleSync = () => {
  clearTimeout(pending);
  pending = setTimeout(sync, 0);
};
const onChange = event => {
  if (owns(event.target)) scheduleSync();
};
const onReset = event => {
  // Reset restores defaultChecked without firing change. Let the native reset
  // finish, then reapply the caller's value without moving focus or emitting a change.
  if (event.target === form) scheduleSync();
};
const observer = new MutationObserver(sync);
observer.observe(group, {subtree: true, childList: true, attributes: true, attributeFilter: ['data-radio-checked']});
group.addEventListener('change', onChange);
form?.addEventListener('reset', onReset, {capture: true});
sync();
try { await dioxus.recv(); }
finally {
  clearTimeout(pending);
  observer.disconnect();
  group.removeEventListener('change', onChange);
  form?.removeEventListener('reset', onReset, {capture: true});
}
