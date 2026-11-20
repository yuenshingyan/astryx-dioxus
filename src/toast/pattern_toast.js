// Private info/default-body Toast controller. Five-second timer, independent
// holds, remaining-time floor, and swipe constants follow pinned Toast sources.
const id = await dioxus.recv();
const row = document.getElementById(id);
if (!row) return;
const card = row.querySelector('.astryx-toast');
const viewport = row.closest('.astryx-demo-toast-viewport');
const media = row.querySelector('.astryx-demo-toast-media');
applyDemoToastMedia(card, media);
const abort = new AbortController();
const listen = (target, name, fn, options = {}) => target.addEventListener(name, fn, {...options, signal:abort.signal});
const holds = new Set();
let remaining = 5000, started = null, timer = null, exitTimer = null, settleTimer = null;
let isDisposed = false, isExiting = false, hasRemoved = false, gesture = null;
const timing = () => parseFloat(getComputedStyle(row).transitionDuration) * 1000;
const start = () => {
  if (holds.size || isExiting || isDisposed) return;
  clearTimeout(timer);
  started = performance.now();
  timer = setTimeout(dismiss, remaining);
};
const pause = source => {
  const wasRunning = !holds.size;
  holds.add(source);
  if (!wasRunning) return;
  clearTimeout(timer);
  timer = null;
  if (started !== null) remaining = Math.max(remaining - (performance.now() - started), 1000);
  started = null;
};
const resume = source => {
  if (!holds.delete(source)) return;
  if (holds.has('focus') && !card.contains(document.activeElement)) holds.delete('focus');
  start();
};
const remove = () => {
  if (hasRemoved || isDisposed) return;
  hasRemoved = true;
  dioxus.send('remove');
};
function dismiss() {
  if (isExiting || isDisposed) return;
  isExiting = true;
  clearTimeout(timer);
  viewport.astryxDemoToast?.handoff(row);
  row.dataset.exiting = '';
  // Transition-end is primary; the fallback also removes in a background tab.
  exitTimer = setTimeout(remove, timing() + 50);
}
listen(row, 'transitionend', event => {
  if (event.target !== row || event.propertyName !== 'grid-template-rows') return;
  if (isExiting) remove(); else row.dataset.settled = '';
});
listen(card, 'mouseenter', () => pause('pointer'));
listen(card, 'mouseleave', () => resume('pointer'));
listen(card, 'focusin', () => pause('focus'));
listen(card, 'focusout', event => {
  if (!card.contains(event.relatedTarget)) resume('focus');
});
listen(window, 'blur', () => pause('window'));
listen(window, 'focus', () => resume('window'));
const clearSwipe = () => {
  card.style.removeProperty('transition-duration');
  for (const key of ['y','exit-y','opacity','scale']) card.style.removeProperty('--_toast-swipe-' + key);
};
const reset = () => { if (isExiting) return; gesture = null; clearSwipe(); resume('gesture'); };
const interactive = target => target.closest('button,a[href],input,select,textarea,[role=button],[role=link],[tabindex],[contenteditable]:not([contenteditable=false])');
const begin = (point, target, source) => {
  if (isExiting || interactive(target)) return;
  if (gesture) { reset(); return; }
  pause('gesture');
  gesture = {x:point.clientX,y:point.clientY,time:performance.now(),height:Math.max(card.getBoundingClientRect().height,1),intent:'pending',source};
};
const move = (point, event) => {
  if (!gesture) return;
  const dx = point.clientX-gesture.x, dy = point.clientY-gesture.y;
  if (gesture.intent === 'pending') {
    if (Math.abs(dx) > 8 && Math.abs(dx) > Math.abs(dy)) { reset(); return; }
    if (Math.abs(dy) <= 8 || Math.abs(dy) <= Math.abs(dx)*1.2) {
      if (gesture.source === 'touch' && event.cancelable) event.preventDefault();
      return;
    }
    if (dy < 0) { reset(); return; }
    gesture.intent = 'vertical';
    card.style.transitionDuration = '0s';
  }
  if (event.cancelable) event.preventDefault();
  const travel = Math.max(0,dy), progress = Math.min(travel/gesture.height,1);
  card.style.setProperty('--_toast-swipe-y',travel+'px');
  card.style.setProperty('--_toast-swipe-opacity',(1-progress*.4).toFixed(3));
  card.style.setProperty('--_toast-swipe-scale',(1-progress*.02).toFixed(3));
};
const end = point => {
  if (!gesture) return;
  const travel = Math.max(0,point.clientY-gesture.y);
  const shouldDismiss = gesture.intent === 'vertical' && (travel >= Math.max(gesture.height*.4,48) || travel/Math.max(1,performance.now()-gesture.time) > 1.2);
  if (shouldDismiss) {
    gesture = null;
    card.style.removeProperty('transition-duration');
    card.style.setProperty('--_toast-swipe-exit-y','120%');
    dismiss();
  } else reset();
};
listen(card, 'pointerdown', event => {
  if (event.pointerType !== 'pen') return;
  begin(event,event.target,'pen');
  if (gesture) card.setPointerCapture(event.pointerId);
});
listen(card, 'pointermove', event => { if (event.pointerType === 'pen') move(event,event); });
listen(card, 'pointerup', event => { if (event.pointerType === 'pen') end(event); });
listen(card, 'pointercancel', reset);
listen(card, 'lostpointercapture', reset);
let touchClaim = 'pending';
listen(card, 'touchstart', event => {
  if (event.touches.length !== 1) { touchClaim = 'rejected'; reset(); return; }
  touchClaim = 'pending';
  begin(event.touches[0],event.target,'touch');
}, {passive:true});
listen(card, 'touchmove', event => {
  if (event.touches.length !== 1) return;
  const point = event.touches[0];
  if (gesture && touchClaim === 'pending') {
    const dx = point.clientX-gesture.x, dy = point.clientY-gesture.y;
    if (dx || dy) touchClaim = dy > 0 && Math.abs(dy) > Math.abs(dx) ? 'claimed' : 'rejected';
  }
  if (touchClaim === 'rejected') { reset(); return; }
  // The pan claim outlives a canceled drawing gesture, as upstream's
  // firstMoveDecides recognizer does; it never returns this touch to scroll.
  if (touchClaim === 'claimed' && event.cancelable) event.preventDefault();
  move(point,event);
}, {passive:false});
listen(card, 'touchend', event => { if (event.changedTouches.length) end(event.changedTouches[0]); touchClaim = 'pending'; });
listen(card, 'touchcancel', () => { touchClaim = 'pending'; reset(); });
const themeObserver = new MutationObserver(() => applyDemoToastMedia(card,media));
for (let ancestor = card.parentElement; ancestor; ancestor = ancestor.parentElement) {
  themeObserver.observe(ancestor, {attributes:true,attributeFilter:['class','style','data-theme','data-astryx-theme']});
}
const colorPreference = matchMedia('(prefers-color-scheme: dark)');
listen(colorPreference, 'change', () => applyDemoToastMedia(card,media));
if (!document.hasFocus()) holds.add('window');
start();
settleTimer = setTimeout(() => { if (!isExiting) row.dataset.settled = ''; }, timing()+50);
// The viewport opens a popover on the first row. Give the browser a frame
// to expose that change before the polite writer schedules its text update.
const announcementFrame = requestAnimationFrame(() => {
  if (isExiting || !row.isConnected) return;
  viewport.astryxDemoToast?.announce(row, row.querySelector('.astryx-demo-toast-body').textContent);
});
try {
  while (true) {
    const command = await dioxus.recv();
    if (command === 'dispose') break;
    if (command === 'dismiss') dismiss();
  }
} finally {
  isDisposed = true;
  abort.abort();
  themeObserver.disconnect();
  cancelAnimationFrame(announcementFrame);
  clearTimeout(timer); clearTimeout(exitTimer); clearTimeout(settleTimer);
  viewport.astryxDemoToast?.handoff(row);
}
