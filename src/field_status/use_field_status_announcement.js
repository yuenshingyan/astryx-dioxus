const marker = await dioxus.recv();
const root = document.querySelector(`[data-astryx-field-status="${marker}"]`);
if (!root) return;
const announcer = installAnnounce().createAnnouncer();
// A containing live region already receives changes to the visible message.
const isEligible = () => {
  if (!root.isConnected) return false;
  for (let element = root.parentElement; element; element = element.parentElement) {
    const live = element.getAttribute('aria-live');
    if (live === 'polite' || live === 'assertive') return false;
    if (live === 'off') return true;
    const role = element.getAttribute('role');
    if (role === 'log' || role === 'status' || role === 'alert') return false;
  }
  return true;
};
try {
  while (true) {
    const next = await dioxus.recv();
    if (next === false) break;
    const [message, isError] = next;
    announcer.withdrawPending();
    if (message) {
      announcer.announce(root, message, isEligible,
        isError ? 'assertive' : 'polite');
    }
  }
} finally {
  announcer.dispose();
}
