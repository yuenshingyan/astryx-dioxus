// Adapted from Astryx useAnnounceRenderedText. See THIRD_PARTY_NOTICES.md.
// This eval stays scoped to its owner; the polite channel stays document-owned.
const renderedWithLoadingPage = document.readyState !== 'complete';
const HIDDEN = '[aria-hidden="true"],[hidden]';
const LIVE_CONTAINER = '[role="log"],[role="status"],[role="alert"],'
  + '[aria-live="polite"],[aria-live="assertive"]';
const BLOCKS = 'p,div,li,br,h1,h2,h3,h4,h5,h6';
const normalizeText = text => text.replace(/\s+/g, ' ').trim();
const renderedText = element => {
  const view = element.ownerDocument.defaultView;
  const displayOf = node => view?.getComputedStyle(node).display ?? '';
  const parts = [];
  const walk = (node, parentDisplay) => {
    if (node.nodeType === 3) {
      parts.push(node.textContent ?? '');
      return;
    }
    if (node.nodeType !== 1 || node.matches(HIDDEN)) return;
    const display = displayOf(node);
    const isBlock = node.matches(BLOCKS)
      || (display !== '' && display !== 'contents' && !display.startsWith('inline'))
      || /\b(flex|grid)\b/.test(parentDisplay);
    if (isBlock) parts.push(' ');
    node.childNodes.forEach(child => walk(child, display));
    if (isBlock) parts.push(' ');
  };
  const display = displayOf(element);
  element.childNodes.forEach(child => walk(child, display));
  return normalizeText(parts.join(''));
};
// Each direct child is a block in skip-first-render mode, including bare text
// beside an icon. Nested inline markup still preserves words such as Fetch+ing.
const blockText = element => [...element.childNodes].map(child => {
  if (child.nodeType === 3) return normalizeText(child.textContent ?? '');
  if (child.nodeType !== 1 || child.matches(HIDDEN)) return '';
  return renderedText(child);
}).filter(Boolean).join(' ');
const announcer = installAnnounce().createAnnouncer();
let config = null;
let currentElement = null;
let lastRevision = null;
let lastElement = null;
let spoken = null;
let isFirstRun = true;
const isEligible = () => config?.[1] && currentElement?.isConnected
  && !currentElement.closest(`${HIDDEN},[inert]`)
  && currentElement.checkVisibility?.({ visibilityProperty: true }) !== false
  && !currentElement.parentElement?.closest(LIVE_CONTAINER);
const update = next => {
  config = next;
  const [selector, isActive, fallbackText, revision] = next;
  // The selector is an internal boundary, but still fail silent for a malformed
  // or non-unique external root rather than reading an unrelated element.
  let matches;
  try {
    matches = document.querySelectorAll(selector);
  } catch {
    announcer.withdrawPending();
    spoken = null;
    return;
  }
  currentElement = matches.length === 1 ? matches[0] : null;
  if (!currentElement) {
    announcer.withdrawPending();
    spoken = null;
    return;
  }
  if (lastRevision === revision && lastElement === currentElement) return;
  lastRevision = revision;
  lastElement = currentElement;
  const isFirst = isFirstRun;
  isFirstRun = false;
  // Dioxus 0.7's hydrator retains this SSR attribute on dynamic/listening
  // roots. onmounted ensures our root is one of them, even after page load.
  const isHydration = isFirst && currentElement.hasAttribute('data-node-hydration');
  if (!isActive || !isEligible()) {
    announcer.withdrawPending();
    spoken = null;
    return;
  }
  let text = blockText(currentElement);
  if (!text) {
    text = normalizeText((currentElement.getAttribute('aria-labelledby') ?? '')
      .split(/\s+/).map(id => {
        const target = currentElement.ownerDocument.getElementById(id);
        return !target || target.closest(HIDDEN) ? '' : renderedText(target);
      }).join(' ')) || normalizeText(fallbackText);
  }
  if (!text || spoken === text) return;
  spoken = text;
  if ((isFirst && renderedWithLoadingPage) || isHydration || document.readyState !== 'complete') return;
  // Bind the root as well as eligibility: a span/div swap must not expose a
  // queued share whose old DOM root has already gone away.
  const element = currentElement;
  announcer.announce(element, text, () => currentElement === element && isEligible());
};
try {
  while (true) {
    const next = await dioxus.recv();
    if (next == null || next === 'dispose') break;
    update(next);
  }
} finally {
  announcer.dispose();
}
