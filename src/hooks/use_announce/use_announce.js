// Adapted from Astryx useAnnounce. See THIRD_PARTY_NOTICES.md.
// Native-dialog fallback and polite writer only; no ModalAnnounceProvider route.
const announceRuntimeKey = Symbol.for('astryx.dioxus.use-announce.polite.v1');
const installAnnounce = () => {
  if (document[announceRuntimeKey]) return document[announceRuntimeKey];
  const hiddenStyle = 'position:absolute;width:1px;height:1px;margin:-1px;padding:0;'
    + 'overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border:0;'
    + 'inset-block-start:0;inset-inline-start:0;pointer-events:none;user-select:none;';
  const states = new WeakMap();
  const isOpenModal = dialog => dialog?.open && dialog.matches(':modal');
  const enclosingModalDialog = element => {
    let dialog = element?.closest('dialog') ?? null;
    while (dialog) {
      if (isOpenModal(dialog)) return dialog;
      dialog = dialog.parentElement?.closest('dialog') ?? null;
    }
    return null;
  };
  const route = element => {
    if (!element.isConnected) return null;
    const modal = enclosingModalDialog(element);
    const open = [...document.querySelectorAll('dialog')].filter(isOpenModal);
    if (!open.length) return document.body;
    if (!modal || modal.closest('[inert],[aria-hidden="true"],[hidden]')) return null;
    // Native showModal keeps focus in the active modal. With multiple native
    // modals and no provider presentation order, fail closed if focus cannot
    // identify it; never guess from document order or announce behind a modal.
    const focused = enclosingModalDialog(document.activeElement);
    if (open.length > 1 && focused !== modal) return null;
    if (focused && focused !== modal) return null;
    return modal;
  };
  const stateFor = container => {
    let state = states.get(container);
    if (state) {
      if (!state.target.isConnected) container.appendChild(state.target);
      return state;
    }
    const target = document.createElement('div');
    target.setAttribute(container === document.body
      ? 'data-astryx-live-region' : 'data-astryx-modal-live-region', 'polite');
    target.setAttribute('role', 'status');
    target.setAttribute('aria-live', 'polite');
    target.setAttribute('aria-atomic', 'true');
    target.style.cssText = hiddenStyle;
    // Initially empty, persistent, and outside the caller's rendered content.
    container.appendChild(target);
    state = { target, container, shares: new Set(), frame: null, settleFrame: null, timer: null };
    states.set(container, state);
    return state;
  };
  const wordsIn = state => [...new Set([...state.shares].map(share => share.text))].join(' ');
  const release = share => {
    share.owner.delete(share);
    share.state.shares.delete(share);
  };
  const cancelFrame = state => {
    if (state.frame !== null) cancelAnimationFrame(state.frame);
    if (state.settleFrame !== null) cancelAnimationFrame(state.settleFrame);
    state.frame = state.settleFrame = null;
  };
  const withdraw = share => {
    const state = share.state;
    if (!state.shares.has(share)) return;
    release(share);
    if (!state.shares.size) {
      cancelFrame(state);
      if (state.timer !== null) clearTimeout(state.timer);
      state.timer = null;
      state.target.textContent = '';
    } else if (state.frame === null) {
      state.target.textContent = wordsIn(state);
    }
  };
  const schedule = state => {
    if (state.frame !== null) return;
    if (state.settleFrame !== null) cancelAnimationFrame(state.settleFrame);
    state.settleFrame = null;
    if (state.timer !== null) clearTimeout(state.timer);
    state.timer = null;
    state.target.textContent = '';
    state.frame = requestAnimationFrame(() => {
      state.frame = null;
      // A native dialog can open/close, or the owner can disappear, between
      // the read and this write. Do not leak its queued words to the page.
      for (const share of [...state.shares]) {
        if (!share.isEligible() || route(share.element) !== state.container) release(share);
      }
      if (!state.shares.size) return;
      state.target.textContent = wordsIn(state);
      // Shares remain withdrawable until the frame after the DOM write, as
      // in the upstream in-dialog writer. Dropping one never drops its peers.
      state.settleFrame = requestAnimationFrame(() => {
        state.settleFrame = null;
        for (const share of [...state.shares]) release(share);
      });
      state.timer = setTimeout(() => {
        state.timer = null;
        state.target.textContent = '';
      }, 2000);
    });
  };
  const runtime = {
    createAnnouncer() {
      const shares = new Set();
      let isDisposed = false;
      const withdrawPending = () => {
        for (const share of [...shares]) withdraw(share);
      };
      return {
        announce(element, text, isEligible) {
          if (isDisposed || !text || !isEligible()) return;
          const container = route(element);
          if (!container) return;
          const state = stateFor(container);
          const share = { element, text, isEligible, owner: shares, state };
          shares.add(share);
          state.shares.add(share);
          schedule(state);
        },
        withdrawPending,
        dispose() {
          isDisposed = true;
          withdrawPending();
        },
      };
    },
  };
  document[announceRuntimeKey] = runtime;
  return runtime;
};
