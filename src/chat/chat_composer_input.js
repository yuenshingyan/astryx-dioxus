const input = document.getElementById(await dioxus.recv());
if (!input) return;
const submit = event => {
  if (event.defaultPrevented || event.key !== 'Enter' || event.shiftKey || event.isComposing || event.keyCode === 229 || input.disabled || input.dataset.submit !== 'true') return;
  event.preventDefault();
  const text = input.value.trim();
  if (text && !event.repeat) dioxus.send(text);
};
document.addEventListener('keydown', onKey);
function onKey(event) {
  if (event.target === input) submit(event);
}
try { await dioxus.recv(); }
finally { document.removeEventListener('keydown', onKey); }
