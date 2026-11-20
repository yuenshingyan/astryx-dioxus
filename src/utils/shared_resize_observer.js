// Adapted from Meta Astryx utils/sharedResizeObserver.ts at
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.
// Realm-local and registration-counted: one observer across Eval channels.
const observeResize = (element, callback, options = null) => {
  const doc = element.ownerDocument;
  const key = Symbol.for('astryx.dioxus.sharedResizeObserver');
  const state = doc[key] ??= { observer: null, callbacks: new Map() };
  const wasObserved = state.callbacks.has(element);
  let callbacks = state.callbacks.get(element);
  if (!callbacks) state.callbacks.set(element, callbacks = new Map());
  callbacks.set(callback, (callbacks.get(callback) || 0) + 1);
  const Observer = doc.defaultView?.ResizeObserver;
  if (!state.observer && Observer) {
    state.observer = new Observer(entries => {
      for (const entry of entries) {
        for (const cb of [...(state.callbacks.get(entry.target)?.keys() || [])]) cb(entry);
      }
    });
  }
  state.observer?.observe(element);
  if (!options?.afterLayout) callback({ target: element });
  let isSubscribed = true;
  if (options?.afterLayout && wasObserved) {
    queueMicrotask(() => { if (isSubscribed) callback(null); });
  }
  return () => {
    if (!isSubscribed) return;
    isSubscribed = false;
    const callbacks = state.callbacks.get(element);
    const count = callbacks?.get(callback) || 0;
    if (count > 1) { callbacks.set(callback, count - 1); return; }
    callbacks?.delete(callback);
    if (callbacks?.size) return;
    state.callbacks.delete(element);
    state.observer?.unobserve(element);
    if (!state.callbacks.size) {
      state.observer?.disconnect();
      state.observer = null;
      delete doc[key];
    }
  };
};
