// Adapted from Text/TruncationTooltip.tsx. Text owns first interest, not presentation.
const createTruncationInterest = ({ records, listen, mount, hover, leave, press, focus, blur, answerTap, remove }) => {
  const wantedKey = Symbol.for('astryx.dioxus.truncation-interest');
  let isTouchInteraction = false;
  const answer = element => element[Symbol.for('astryx.dioxus.truncation')];
  const isCut = record => answer(record.element)?.isTruncated ??
    (record.element.scrollWidth > record.element.clientWidth || record.element.scrollHeight > record.element.clientHeight);
  const text = record => answer(record.element)?.fullText ?? record.element.textContent ?? record.label;
  const candidates = target => [...records.values()].filter(record => record.isTruncation && record.element.contains(target));
  const want = record => {
    if (!isCut(record)) return false;
    record.isTruncationWanted = true;
    record.element[wantedKey] = true;
    return mount(record, text(record));
  };
  const refresh = record => {
    if (!isCut(record)) { remove(record); return; }
    if (record.isTruncationWanted) mount(record, text(record));
  };
  listen(document, 'pointerover', event => {
    if (event.pointerType === 'touch') return;
    for (const record of candidates(event.target)) {
      if (record.element.contains(event.relatedTarget) || !want(record)) continue;
      hover(record, () => isCut(record) ? text(record) : null);
    }
  }, { capture: true });
  listen(document, 'pointerout', event => {
    for (const record of candidates(event.target)) {
      if (record.element.contains(event.relatedTarget)) continue;
      leave(record, event.relatedTarget);
    }
  }, { capture: true });
  listen(document, 'focusin', event => {
    if (isTouchInteraction || !event.target.matches?.(':focus-visible')) return;
    for (const record of candidates(event.target)) {
      if (!want(record)) continue;
      focus(record, event.target, text(record));
    }
  }, { capture: true });
  listen(document, 'focusout', event => {
    for (const record of candidates(event.target)) {
      blur(record, event.target);
    }
  }, { capture: true });
  const tapRecognizer = createTruncationTapRecognizer({ listen, onTap(record, event) {
    if (want(record)) answerTap(record, event.target, text(record));
  } });
  listen(document, 'pointerdown', event => {
    isTouchInteraction = event.pointerType === 'touch' || event.pointerType === 'pen';
    tapRecognizer.forget();
    for (const record of candidates(event.target)) {
      press(record, isTouchInteraction);
      if (isTouchInteraction) tapRecognizer.press(event, record);
    }
  }, { capture: true, passive: true });
  listen(document, 'keydown', event => {
    if (['Tab', 'Escape', 'ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(event.key) || !event.target.closest?.('input,textarea,[contenteditable]')) isTouchInteraction = false;
  }, { capture: true });
  return {
    attach(record) {
      if (!record.isTruncation) return;
      record.isTruncationWanted = record.element[wantedKey] === true;
      record.truncationAbort = new AbortController();
      record.element.addEventListener('astryxtruncationchange', () => refresh(record), { signal: record.truncationAbort.signal });
      if (record.isTruncationWanted) refresh(record);
    },
    dispose(record) {
      tapRecognizer.dispose(record);
      record.truncationAbort?.abort();
      if (!record.element.isConnected) delete record.element[wantedKey];
    },
  };
};
// Private useCutTextOnKeyboardFocus adapter; no measurements at rest.
const cutTextOnKeyboardFocus = element => {
  const fields = 'input,textarea,select,[contenteditable]:not([contenteditable="false"]),[role="combobox"],[role="listbox"],[role="searchbox"],[role="spinbutton"],[role="slider"],[role="textbox"]';
  const isLeftOut = node => {
    if (node.closest(`[hidden],[aria-hidden="true"],${fields}`)) return true;
    const css = getComputedStyle(node);
    return css.position === 'absolute' && ((css.clip && css.clip !== 'auto') || (css.clipPath && css.clipPath !== 'none'));
  };
  const overflows = node => (node.clientWidth || node.offsetWidth) > 0 && node.scrollWidth > (node.clientWidth || node.offsetWidth);
  if (!overflows(element) && ![...element.querySelectorAll('*')].some(node => !isLeftOut(node) && overflows(node))) return null;
  let text = '';
  const walk = node => {
    if (node.nodeType === Node.TEXT_NODE) text += node.textContent;
    else if (node instanceof Element && !isLeftOut(node)) node.childNodes.forEach(walk);
  };
  walk(element);
  return text.replace(/\s+/g, ' ').trim() || null;
};
