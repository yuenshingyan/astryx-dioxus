// Adapted from Astryx Spinner/spinnerRotor.ts. See THIRD_PARTY_NOTICES.md.
const rotorId = await dioxus.recv();
const rotor = Array.from(document.querySelectorAll('[data-astryx-spinner-rotor-id]'))
  .find(element => element.getAttribute('data-astryx-spinner-rotor-id') === rotorId);
if (rotor && typeof rotor.getAnimations === 'function') {
  const key = Symbol.for('astryx.dioxus.spinner.rotors');
  const state = document[key] ??= {pending: new Set(), frame: null};
  state.pending.add(rotor);
  if (state.frame === null) {
    state.frame = requestAnimationFrame(() => {
      state.frame = null;
      const animations = [];
      for (const element of state.pending) {
        if (element.isConnected) animations.push(...element.getAnimations());
      }
      state.pending.clear();
      // Read every animation first, then write start times: no read/write thrash.
      for (const animation of animations) animation.startTime = 0;
    });
  }
  try {
    while (true) {
      const update = await dioxus.recv();
      if (update == null || update === 'dispose') break;
    }
  } finally {
    state.pending.delete(rotor);
    if (state.pending.size === 0 && state.frame !== null) {
      cancelAnimationFrame(state.frame);
      state.frame = null;
    }
  }
}
