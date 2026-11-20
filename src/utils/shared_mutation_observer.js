// Adapted from Meta Astryx utils/sharedMutationObserver.ts at
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.
// Content-only form: one realm-local observer, independent registrations.
const observeContent = (element, callback) => {
  const doc = element.ownerDocument || element;
  const Observer = doc.defaultView?.MutationObserver;
  if (!Observer) return () => {};
  const key = Symbol.for('astryx.dioxus.sharedContentObserver');
  let state = doc[key];
  if (!state) {
    const callbacks = new Map();
    const observer = new Observer(records => {
      const pending = new Set();
      for (const record of records) {
        for (let node = record.target; node; node = node.parentNode) {
          for (const cb of callbacks.get(node) || []) pending.add(cb);
        }
      }
      for (const cb of pending) cb();
    });
    state = doc[key] = { callbacks, observer };
  }
  let callbacks = state.callbacks.get(element);
  if (!callbacks) {
    state.callbacks.set(element, callbacks = new Set());
    state.observer.observe(element, { childList: true, subtree: true, characterData: true });
  }
  callbacks.add(callback);
  let isSubscribed = true;
  return () => {
    if (!isSubscribed) return;
    isSubscribed = false;
    callbacks.delete(callback);
    if (!callbacks.size) state.callbacks.delete(element);
    if (!state.callbacks.size) {
      state.observer.disconnect();
      if (doc[key] === state) delete doc[key];
    }
  };
};
