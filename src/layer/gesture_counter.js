// Private tap recognizer adapted from Layer/gestureCounter.ts. Only a completed
// press's click answers a tap; a new press/key/scroll invalidates the old verdict.
const createTruncationTapRecognizer = ({ listen, onTap }) => {
  let tap = null;
  listen(document, 'pointermove', event => {
    if (tap?.id === event.pointerId && Math.hypot(event.clientX - tap.x, event.clientY - tap.y) > 10) tap = null;
  }, { capture: true, passive: true });
  const end = event => {
    if (tap?.id !== event.pointerId) return;
    if (event.type === 'pointercancel' || !tap.record.element.contains(event.target)) { tap = null; return; }
    tap.isReleased = true;
  };
  listen(document, 'pointerup', end, { capture: true });
  listen(document, 'pointercancel', end, { capture: true });
  listen(document, 'click', event => {
    const completed = tap;
    tap = null;
    if (completed?.isReleased && completed.record.element.isConnected && completed.record.element.contains(event.target)) onTap(completed.record, event);
  }, { capture: true });
  listen(document, 'keydown', () => { tap = null; }, { capture: true });
  listen(document, 'scroll', () => { tap = null; }, { capture: true, passive: true });
  return {
    forget() { tap = null; },
    press(event, record) {
      if (event.isPrimary && event.button === 0) tap = { record, id: event.pointerId, x: event.clientX, y: event.clientY, isReleased: false };
    },
    dispose(record) { if (tap?.record === record) tap = null; },
  };
};
