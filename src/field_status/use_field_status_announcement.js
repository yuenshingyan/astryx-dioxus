const marker = await dioxus.recv();
const root = document.querySelector(`[data-astryx-field-status="${marker}"]`);
if (!root) return;
const announcer = installAnnounce().createAnnouncer();
try {
  while (true) {
    const next = await dioxus.recv();
    if (next === false) break;
    const [message, isError] = next;
    announcer.withdrawPending();
    if (message) {
      announcer.announce(root, message, () => root.isConnected,
        isError ? 'assertive' : 'polite');
    }
  }
} finally {
  announcer.dispose();
}
