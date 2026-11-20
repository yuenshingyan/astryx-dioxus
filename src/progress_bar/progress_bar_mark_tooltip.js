// ProgressBarMarkTooltip owns only the native anchor binding.
const initial = await dioxus.recv();
if (initial == null || initial === 'dispose') return;
const [id, label] = initial;
const element = document.querySelector(`[data-astryx-progress-mark-id="${CSS.escape(id)}"]`);
if (!element) return;
installPressFeedback();
const controller = document[runtimeKey].attach(element, {
  id, label, tooltip: label, isIconOnly: false, isTruncation: false,
  isDisabled: false, isLoading: false, focusTriggerAlways: true,
});
const click = () => controller.preview();
element.addEventListener('click', click);
// Unlike React's commit selection restoration, a keyed Dioxus DOM move can
// blur its existing anchor. Restore only a same-batch remove/reinsert, never
// a deliberate blur, a removed mark, or focus transferred to another control.
let isBlurPending = false, wasRemoved = false, wasAdded = false, isDisposed = false;
const collectMoves = records => {
  if (!isBlurPending) return;
  for (const record of records) {
    wasRemoved ||= [...record.removedNodes].includes(element);
    wasAdded ||= [...record.addedNodes].includes(element);
  }
};
const focusObserver = new MutationObserver(collectMoves);
const focus = () => {
  focusObserver.disconnect();
  isBlurPending = wasRemoved = wasAdded = false;
  if (element.parentElement) focusObserver.observe(element.parentElement, { childList: true });
};
const blur = () => {
  isBlurPending = true;
  queueMicrotask(() => {
    collectMoves(focusObserver.takeRecords());
    const shouldRestore = !isDisposed && isBlurPending && wasRemoved && wasAdded
      && element.isConnected && element.ownerDocument.activeElement === element.ownerDocument.body;
    focusObserver.disconnect();
    isBlurPending = false;
    if (shouldRestore) element.focus({ preventScroll: true });
  });
};
element.addEventListener('focus', focus);
element.addEventListener('blur', blur);
if (element.ownerDocument.activeElement === element) focus();
try {
  while (true) {
    const update = await dioxus.recv();
    if (update == null || update === 'dispose') break;
    const [, label] = update;
    controller.update({ label, tooltip: label });
  }
} finally {
  isDisposed = true;
  focusObserver.disconnect();
  element.removeEventListener('focus', focus);
  element.removeEventListener('blur', blur);
  element.removeEventListener('click', click);
  controller.dispose();
}
