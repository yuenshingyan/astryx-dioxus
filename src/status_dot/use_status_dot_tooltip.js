// Renderer channel only. Tooltip owns placement/holds; Press owns manager lifetime.
const initial = await dioxus.recv();
if (initial == null || initial === 'dispose') return;
const [id, label, tooltip, callerAttributeNames] = initial;
if (!tooltip?.trim()) return;
const element = document.querySelector(`[data-astryx-status-tooltip-id="${CSS.escape(id)}"]`);
if (!element) return;
installAnnounce();
installPressFeedback();
const controller = document[runtimeKey].attach(element, {
  id, label, tooltip, callerAttributeNames,
  tooltipPolicyKey: 'status-dot', tooltipPolicyFactory: createStatusDotTooltipSupport,
  isTruncation: false, isIconOnly: false, isLoading: false, isDisabled: false,
  // The shared manager may have been created by an earlier Button eval. Pass
  // this utility capability explicitly; eval lexical scopes are independent.
  holdHitArea: holdTouchHitArea,
  reportPlacement: value => dioxus.send(value),
});
try {
  while (true) {
    const update = await dioxus.recv();
    if (update == null || update === 'dispose') break;
    if (typeof update === 'string') { controller.command(update); continue; }
    const [, label, tooltip, callerAttributeNames] = update;
    controller.update({ label, tooltip, callerAttributeNames });
  }
} finally {
  controller.dispose();
}
