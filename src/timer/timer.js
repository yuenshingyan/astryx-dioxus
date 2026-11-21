const [id, initialConfig] = await dioxus.recv();
const node = document.querySelector(`[data-astryx-timer-id="${CSS.escape(id)}"]`);
if (!node) return;
const mountedAt = Date.now();
const pad = value => String(value).padStart(2, '0');
let timeout;

const schedule = ([start, isClock]) => {
  clearTimeout(timeout);
  const startTime = start ?? mountedAt;
  const tick = () => {
    if (!node.isConnected) return;
    const now = Date.now();
    const elapsed = Math.max(0, now - startTime);
    const seconds = Math.floor(elapsed / 1000);
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor(seconds / 60);
    const hasMinutePrecision = !isClock && hours > 0;
    const text = isClock
      ? `${hours ? `${hours}:${pad(minutes % 60)}` : minutes}:${pad(seconds % 60)}`
      : hasMinutePrecision ? `${hours}h ${pad(minutes % 60)}m`
        : minutes ? `${minutes}m ${pad(seconds % 60)}s` : `${seconds}s`;
    const dateTime = `PT${hasMinutePrecision ? minutes * 60 : seconds}S`;
    // Keep the text node registered with the renderer intact.
    const textNode = node.firstChild;
    if (textNode?.nodeType === Node.TEXT_NODE && textNode.data !== text) textNode.data = text;
    if (node.dateTime !== dateTime) node.dateTime = dateTime;
    const precision = hasMinutePrecision ? 60_000 : 1000;
    const delay = now < startTime ? Math.min(startTime - now + 1000, 2_147_483_647)
      : precision - elapsed % precision;
    timeout = setTimeout(tick, Math.ceil(delay));
  };
  tick();
};

try {
  schedule(initialConfig);
  while (true) {
    const config = await dioxus.recv();
    if (config === false) break;
    schedule(config);
  }
} finally {
  clearTimeout(timeout);
}
