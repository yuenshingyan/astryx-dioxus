// ProgressBarTouch owns native retargeting, not rendering or paint.
// Adapted from Meta Astryx 476a3153417dbf75c565f7df05b0784e94ddd59e;
// ProgressBar.spec.md supersedes measured touch timing. See THIRD_PARTY_NOTICES.md.
const initial = await dioxus.recv();
if (!Array.isArray(initial)) return;
const [id] = initial;
const root = [...document.querySelectorAll('[data-astryx-progress-bar-id]')]
  .find(node => node.getAttribute('data-astryx-progress-bar-id') === id);
if (!root) return;
const doc = root.ownerDocument, view = doc.defaultView;
if (!view) return;
const own = selector => [...root.querySelectorAll(selector)]
  .filter(node => node.closest('.astryx-progress-bar') === root);
let fill = null, track = null, config = null, animation = null;
let releaseResize = null, markObserver = null, releases = [], isDisposed = false;
const attributes = new Map(), properties = new Map(), placements = new Map();
const attr = (name, value) => {
  if (!attributes.has(name)) attributes.set(name, { original: root.getAttribute(name), last: null });
  attributes.get(name).last = value;
  if (root.getAttribute(name) !== value) root.setAttribute(name, value);
};
const property = (name, value) => {
  if (!properties.has(name)) properties.set(name, {
    original: root.style.getPropertyValue(name), priority: root.style.getPropertyPriority(name), last: null,
  });
  properties.get(name).last = value;
  if (root.style.getPropertyValue(name) !== value) root.style.setProperty(name, value);
};
const cancelMove = () => {
  if (!animation) return;
  animation.onfinish = null;
  animation.cancel();
  animation = null;
};
const clearMode = () => {
  cancelMove();
  releaseResize?.(); releaseResize = null;
  markObserver?.disconnect(); markObserver = null;
  releases.forEach(release => release()); releases = [];
};
const restorePlacements = () => {
  for (const [mark, entry] of placements) {
    if (mark.getAttribute('data-placement') !== entry.last) continue;
    if (entry.original === null) mark.removeAttribute('data-placement');
    else mark.setAttribute('data-placement', entry.original);
  }
  placements.clear();
};
const media = query => view.matchMedia?.(query) || { matches: false };
const coarse = media(TOUCH_QUERY);
const reduced = media('(prefers-reduced-motion: reduce)');
const forced = media('(forced-colors: active)');
const listenMedia = (query, callback) => {
  if (query.addEventListener) {
    query.addEventListener('change', callback);
    releases.push(() => query.removeEventListener('change', callback));
  } else if (query.addListener) {
    query.addListener(callback);
    releases.push(() => query.removeListener(callback));
  }
};
const normalized = value => Number.isFinite(value) ? Math.min(100, Math.max(0, value)) : 0;
// The source minimum is two of ProgressBar's fixed 8px track heights, not
// UIKit's measured 4pt height. CSS applies this minimum without measuring.
const minimum = progressBar.progressView.minimumFillHeights * 8;
let trackWidth = 0;
const syncMarks = () => {
  if (!config || config.mode !== 'determinate' || !config.hasMarks) return;
  const drawn = coarse.matches && config.percent > 0 && trackWidth > 0
    ? Math.min(100, Math.max(config.percent, minimum / trackWidth * 100)) : config.percent;
  const marks = new Set(own('.astryx-progress-bar-mark'));
  for (const mark of placements.keys()) if (!marks.has(mark)) placements.delete(mark);
  for (const mark of marks) {
    const text = mark.getAttribute('data-astryx-mark-percent');
    const percent = text === null || text.trim() === '' ? NaN : Number(text);
    if (!Number.isFinite(percent)) continue;
    const placement = config.percent > 0 && normalized(percent) <= drawn ? 'fill' : 'track';
    if (!placements.has(mark)) placements.set(mark, { original: mark.getAttribute('data-placement'), last: null });
    placements.get(mark).last = placement;
    if (mark.getAttribute('data-placement') !== placement) mark.setAttribute('data-placement', placement);
  }
};
const syncResize = () => {
  const shouldWatch = config?.mode === 'determinate' && config.hasMarks && coarse.matches;
  if (!shouldWatch) {
    releaseResize?.(); releaseResize = null; trackWidth = 0;
    markObserver?.disconnect(); markObserver = null;
    if (!config?.hasMarks) restorePlacements();
  } else if (!releaseResize) {
    releaseResize = observeResize(track, entry => {
      if (isDisposed || !track.isConnected) return;
      trackWidth = entry.contentRect?.width ?? track.getBoundingClientRect().width;
      syncMarks();
    });
    // Mark-only renders need not change the hook's percentage/has_marks.
    // Watch only our normalized geometry and placement, never mark labels.
    markObserver = new view.MutationObserver(syncMarks);
    markObserver.observe(root, { childList: true, subtree: true, attributes: true,
      attributeFilter: ['data-astryx-mark-percent', 'data-placement'] });
  }
  syncMarks();
};
const syncDirection = () => {
  const direction = view.getComputedStyle(track).direction === 'rtl' ? 'rtl' : 'ltr';
  if (root.getAttribute('data-astryx-progress-bar-direction') !== direction) {
    cancelMove();
    attr('data-astryx-progress-bar-direction', direction);
  }
};
const syncFields = () => {
  syncDirection();
  attr('data-astryx-progress-bar-empty', String(config.percent === 0));
  property('--_progress-bar-drawn', `${config.percent}%`);
  property('--_progress-bar-min', `${minimum}px`);
  property('--_progress-bar-pulse', `${progressBarMotionRoles['progress.reducedMotionPulseMs']}ms`);
  property('--_progress-bar-pulse-floor', String(progressBar.reducedMotionPulseFloor));
  property('--_progress-bar-pulse-curve', `cubic-bezier(${timing.curves.move.join(', ')})`);
};
const reconcile = (next, isInitial = false) => {
  const nextTrack = own('.astryx-progress-bar-track')[0];
  const nextFill = own('.astryx-progress-bar-fill')[0];
  if (!nextTrack || !nextFill) { clearMode(); return; }
  const hasModeChanged = config?.mode !== next.mode || fill !== nextFill || track !== nextTrack;
  const hasValueChanged = config?.percent !== next.percent;
  if (!hasModeChanged && !hasValueChanged) {
    config = next;
    syncFields(); syncResize();
    return;
  }
  // A renderer can replace the caller's root style attribute. Reinstate only
  // our old drawn endpoint BEFORE sampling; the new model --fill is not it.
  if (config) property('--_progress-bar-drawn', `${config.percent}%`);
  const from = !hasModeChanged && config?.mode === 'determinate'
    ? view.getComputedStyle(fill).transform : null;
  if (hasModeChanged) { clearMode(); restorePlacements(); }
  else cancelMove();
  track = nextTrack; fill = nextFill; config = next;
  attr('data-astryx-progress-bar-motion', 'active');
  syncFields();
  syncResize();
  if (hasModeChanged && next.mode === 'determinate') {
    const onPreference = () => { cancelMove(); syncFields(); syncResize(); };
    listenMedia(coarse, onPreference);
    listenMedia(reduced, onPreference);
    listenMedia(forced, onPreference);
  }
  // Forced colors need a sized fill with a whole outline. Both pointers land
  // immediately there, rather than animating width/layout each frame.
  if (isInitial || hasModeChanged || next.mode !== 'determinate' || reduced.matches || forced.matches || !from) return;
  const to = view.getComputedStyle(fill).transform;
  if (from === to || typeof fill.animate !== 'function') return;
  const { duration, easing } = readThemeTimingTokens(fill, '--duration-medium', '--ease-standard');
  if (duration <= 0) return;
  // A new WAAPI effect always gets the FULL theme clock, including returns
  // to its previous start. No CSS reversal shortening or pixel-jitter hack.
  animation = fill.animate([{ transform: from }, { transform: to }], { duration, easing });
  const current = animation;
  current.onfinish = () => { if (animation === current) cancelMove(); };
};
const decode = value => {
  if (!Array.isArray(value) || value[0] !== id || !['determinate', 'indeterminate'].includes(value[2])) return null;
  return { percent: normalized(value[1]), mode: value[2], hasMarks: value[3] === true };
};
let resolveRemoval;
const removal = new Promise(resolve => { resolveRemoval = resolve; });
// Child-list observation checks only our connectivity. Direction changes
// re-read only our track's rendered region, never an RTL ancestor selector.
// Neither path inspects labels, text, or external content relationships.
const observer = new view.MutationObserver(records => {
  if (!root.isConnected) { dispose(); resolveRemoval(null); return; }
  for (const mark of placements.keys()) if (!mark.isConnected || !root.contains(mark)) placements.delete(mark);
  if (track && records.some(record => record.type === 'attributes')) syncDirection();
});
const dispose = () => {
  if (isDisposed) return;
  isDisposed = true;
  clearMode(); restorePlacements(); observer.disconnect();
  for (const [name, entry] of properties) {
    if (root.style.getPropertyValue(name) !== entry.last) continue;
    if (entry.original) root.style.setProperty(name, entry.original, entry.priority);
    else root.style.removeProperty(name);
  }
  for (const [name, entry] of attributes) {
    if (root.getAttribute(name) !== entry.last) continue;
    if (entry.original === null) root.removeAttribute(name);
    else root.setAttribute(name, entry.original);
  }
};
try {
  observer.observe(doc.documentElement, { childList: true, subtree: true, attributes: true, attributeFilter: ['dir'] });
  const next = decode(initial);
  if (!next) return;
  reconcile(next, true);
  while (!isDisposed) {
    const update = await Promise.race([dioxus.recv(), removal]);
    if (update == null || update === 'dispose') break;
    const next = decode(update);
    if (!next) break;
    reconcile(next);
  }
} finally {
  dispose();
}
