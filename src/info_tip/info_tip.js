// InfoTip owns activation and trigger semantics; the shared manager owns the surface.
const createInfoTipPolicy = ({ show, hide }) => ({
  attach() {},
  update() {},
  dispose() {},
  isHoverOnly() { return false; },
  isEnabled(record) { return !record.element.closest('[inert],:disabled'); },
  onShow(record) {
    record.element.setAttribute('aria-expanded', String(!!record.isPinned));
    record.element.setAttribute('aria-controls', record.tip.id);
  },
  onHide(record) {
    record.element.setAttribute('aria-expanded', 'false');
    record.element.removeAttribute('aria-controls');
  },
  command(record, command) {
    if (command !== 'toggle') return;
    if (record.isPinned) hide(record);
    else show(record);
  },
});
const [id, tooltip] = await dioxus.recv();
const element = document.querySelector(`[data-astryx-info-tip-id="${CSS.escape(id)}"]`);
if (!element) return;
installAnnounce();
installPressFeedback();
const controller = document[runtimeKey].attach(element, {
  id, tooltip, tooltipPolicyKey: 'info-tip', tooltipPolicyFactory: createInfoTipPolicy,
  isTruncation: false, isIconOnly: true, isLoading: false, isDisabled: false,
});
try {
  while (true) {
    const command = await dioxus.recv();
    if (command == null || command === 'dispose') break;
    if (command === 'toggle') controller.command(command);
    else controller.update({tooltip: command[0]});
  }
} finally { controller.dispose(); }
