// Shared renderer-channel lifetime for pressables and Tooltip interest anchors.
const initial = await dioxus.recv();
if (initial == null || initial === 'dispose') return;
const [id, label, tooltip, isIconOnly, isLoading, isDisabled, isTruncation = false] = initial;
const escapedId = CSS.escape(id);
const element = document.querySelector(`[data-astryx-button-id="${escapedId}"],[data-astryx-feedback-id="${escapedId}"],[data-astryx-truncation-id="${escapedId}"]`);
if (!element) return;
installPressFeedback();
const controller = document[runtimeKey].attach(element, {
  id, label, tooltip, isIconOnly, isLoading, isDisabled, isTruncation,
  ...pressCompatibility(element),
});
try {
  while (true) {
    const update = await dioxus.recv();
    if (update == null || update === 'dispose') break;
    const [, label, tooltip, isIconOnly, isLoading, isDisabled, isTruncation = false] = update;
    controller.update({ label, tooltip, isIconOnly, isLoading, isDisabled, isTruncation });
  }
} finally {
  controller.dispose();
}
