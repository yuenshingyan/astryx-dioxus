// Button-owned loading-rotor compatibility for the shared press bridge.
// Adapted from Astryx Button/spinnerRotor. See THIRD_PARTY_NOTICES.md.
const pressCompatibility = element => {
  if (!element.matches('.astryx-button')) return {};
  return {
    onUpdate(record) {
      if (!record.element.querySelector('[data-astryx-spinner-rotor]')) return;
      if (record.frame) cancelAnimationFrame(record.frame);
      record.frame = requestAnimationFrame(() => {
        record.frame = null;
        record.element.querySelectorAll('[data-astryx-spinner-rotor]').forEach(rotor => {
          rotor.getAnimations().forEach(animation => { animation.startTime = 0; });
        });
      });
    },
  };
};
