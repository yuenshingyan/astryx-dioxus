// Adapted from Astryx Tooltip/useTooltip. See THIRD_PARTY_NOTICES.md.
// Tooltip owns presentation, descriptions and interest; press only notifies dismissal.
const createTooltipController = ({ records, listen, later, cancelTimer }) => {
  let visibleTip = null;
  const hideTip = record => {
    if (!record) return;
    record.cutResize?.disconnect();
    record.cutMutation?.disconnect();
    cancelTimer(record, record.showTimer);
    cancelTimer(record, record.hideTimer);
    if (record.tip) {
      const id = record.tip.id;
      if (record.isAutomatic) {
        record.tip.remove();
        record.layerAbort?.abort();
        record.boundTip = null;
        const ids = (record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(value => value && value !== id);
        if (ids.length) record.element.setAttribute('aria-describedby', ids.join(' '));
        else record.element.removeAttribute('aria-describedby');
      } else if (record.tip.matches(':popover-open')) {
        record.tip.hidePopover();
      }
      record.tip = null;
    }
    if (visibleTip === record) visibleTip = null;
  };
  const positionTip = record => {
    if (!record.tip?.isConnected || !record.element.isConnected) return;
    const box = record.element.getBoundingClientRect();
    const tipBox = record.tip.getBoundingClientRect();
    const viewport = visualViewport;
    const left = viewport?.offsetLeft || 0;
    const top = viewport?.offsetTop || 0;
    const width = viewport?.width || innerWidth;
    const height = viewport?.height || innerHeight;
    const above = box.top - tipBox.height - 8;
    record.tip.style.left = `${Math.max(left + 4, Math.min(box.left + (box.width - tipBox.width) / 2, left + width - tipBox.width - 4))}px`;
    record.tip.style.top = `${Math.max(top + 4, Math.min(above >= top + 4 ? above : box.bottom + 8, top + height - tipBox.height - 4))}px`;
  };
  const accessibleName = element => {
    if (element.hasAttribute('aria-labelledby')) return null;
    if (element.hasAttribute('aria-label')) return element.getAttribute('aria-label');
    const content = (element.querySelector('.astryx-button__content') || element).cloneNode(true);
    content?.querySelectorAll('[aria-hidden="true"]').forEach(node => node.remove());
    return content?.textContent || '';
  };
  const syncDescription = record => {
    const id = `${record.id}-tooltip`;
    const ids = new Set((record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean));
    const name = record.tooltip ? accessibleName(record.element) : null;
    if (!record.tooltip || (name !== null && name.trim() === record.tooltip.trim())) ids.delete(id);
    else ids.add(id);
    if (ids.size) record.element.setAttribute('aria-describedby', [...ids].join(' '));
    else record.element.removeAttribute('aria-describedby');
  };
  const callerTooltip = record => (record.element.getAttribute('aria-describedby') || '').split(/\s+/)
    .some(id => id && document.getElementById(id)?.getAttribute('role') === 'tooltip' && id !== record.tip?.id);
  const showTip = (record, text, automatic = false) => {
    if (!text || !record.element.isConnected || !records.has(record.element)) return;
    if (visibleTip && visibleTip !== record) hideTip(visibleTip);
    hideTip(record);
    const existing = !automatic && document.getElementById(`${record.id}-tooltip`);
    const tip = existing || document.createElement('div');
    tip.id = `${record.id}-tooltip`;
    tip.className = 'astryx-tooltip astryx-button-tooltip';
    tip.setAttribute('role', 'tooltip');
    tip.setAttribute('popover', 'manual');
    tip.textContent = text;
    // Copy inherited theme/typography onto the top-layer surface, not body defaults.
    const css = getComputedStyle(record.element);
    ['--color-background-inverted', '--color-background-surface', '--radius-container', '--text-label-size', '--text-label-leading'].forEach(name => tip.style.setProperty(name, css.getPropertyValue(name)));
    tip.style.fontFamily = css.fontFamily;
    if (!existing) {
      // Automatic Text tips must not become part of an enclosing link's name.
      const host = automatic && record.isTruncation
        ? record.element.closest('a,button,[role="link"],[role="button"]') || record.element
        : record.element;
      host.after(tip);
      if (!automatic) record.ownedTip = tip;
    }
    record.tip = tip;
    record.isAutomatic = automatic;
    const name = accessibleName(record.element);
    if (!automatic && (name === null || name.trim() !== text.trim())) {
      const ids = new Set((record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean));
      ids.add(tip.id);
      record.element.setAttribute('aria-describedby', [...ids].join(' '));
    }
    tip.showPopover?.();
    positionTip(record);
    if (automatic && record.isTruncation) {
      const refresh = () => {
        if (!record.tip || !record.element.isConnected) return;
        const element = record.element;
        if (element.scrollWidth <= element.clientWidth && element.scrollHeight <= element.clientHeight) {
          hideTip(record); return;
        }
        const text = element.textContent || record.label;
        if (record.tip.textContent !== text) record.tip.textContent = text;
        positionTip(record);
      };
      record.cutResize = new ResizeObserver(refresh);
      record.cutMutation = new MutationObserver(refresh);
      record.cutResize.observe(record.element);
      record.cutMutation.observe(record.element, { childList: true, subtree: true, characterData: true });
    }
    if (record.boundTip !== tip) {
      record.layerAbort?.abort();
      record.layerAbort = new AbortController();
      record.boundTip = tip;
      tip.addEventListener('pointerenter', () => cancelTimer(record, record.hideTimer), { signal: record.layerAbort.signal });
      tip.addEventListener('pointerleave', () => { record.hideTimer = later(record, () => hideTip(record), 100); }, { signal: record.layerAbort.signal });
    }
    visibleTip = record;
  };
  const interestRecord = target => {
    let nearest = null;
    for (let element = target; element; element = element.parentElement) {
      const record = records.get(element);
      if (record?.tooltip) return record;
      if (record && !nearest) nearest = record;
    }
    return nearest;
  };
  const showAutomatic = record => {
    if (!record.isTruncation || callerTooltip(record)) return;
    // An explicit parent tooltip already explains this label. Do not stack tips.
    for (let parent = record.element.parentElement; parent; parent = parent.parentElement) {
      if (records.get(parent)?.tooltip || callerTooltip({ element: parent })) return;
    }
    const element = record.element;
    if (getComputedStyle(element).visibility !== 'hidden'
      && (element.scrollWidth > element.clientWidth || element.scrollHeight > element.clientHeight)) {
      showTip(record, element.textContent || record.label, true);
    }
  };
  const focusTip = record => {
    if (record.pointerFocus) return;
    if (record.isTruncation && !document.activeElement?.matches(':focus-visible')) return;
    if (record.tooltip) { showTip(record, record.tooltip); return; }
    if (record.isTruncation) { showAutomatic(record); return; }
    if (record.isIconOnly || !record.element.matches(':focus-visible') || callerTooltip(record)) return;
    const label = record.element.querySelector('.astryx-button__label');
    if (label && getComputedStyle(label).visibility !== 'hidden' && label.scrollWidth > label.clientWidth) {
      showTip(record, record.label, true);
    }
  };
  listen(document, 'focusin', event => {
    const record = interestRecord(event.target);
    if (record) { record.isFocused = true; focusTip(record); }
    if (!record?.tooltip && !record?.pointerFocus && event.target.matches?.(':focus-visible')) {
      const text = [...records.values()].find(item => item.isTruncation && event.target.contains(item.element));
      if (text) { text.isFocused = true; focusTip(text); }
    }
  }, { capture: true });
  listen(document, 'focusout', event => {
    for (const item of records.values()) {
      if (item.isTruncation && event.target.contains(item.element)) { item.isFocused = false; item.pointerFocus = false; hideTip(item); }
    }
    const record = interestRecord(event.target);
    if (record) {
      record.hadFocus = record.isFocused;
      record.isFocused = false; record.pointerFocus = false; hideTip(record);
      queueMicrotask(() => { if (record.element.isConnected) record.hadFocus = false; });
    }
  }, { capture: true });
  listen(document, 'pointerover', event => {
    const record = interestRecord(event.target);
    if (!record || record.element.contains(event.relatedTarget) || event.pointerType === 'touch' || (!record.tooltip && !record.isTruncation)) return;
    cancelTimer(record, record.hideTimer);
    cancelTimer(record, record.showTimer);
    record.showTimer = later(record, () => record.tooltip ? showTip(record, record.tooltip) : showAutomatic(record), 200);
  }, { capture: true });
  listen(document, 'pointerout', event => {
    const record = interestRecord(event.target);
    if (!record || record.element.contains(event.relatedTarget)) return;
    cancelTimer(record, record.showTimer);
    if (record.tip && !record.isFocused) record.hideTimer = later(record, () => hideTip(record), 100);
  }, { capture: true });
  let tap = null;
  listen(document, 'pointerdown', event => {
    const record = interestRecord(event.target);
    if (record) { record.pointerFocus = true; hideTip(record); }
    tap = event.isPrimary && event.button === 0 && event.pointerType === 'touch' && record?.isTruncation
      ? { record, id: event.pointerId, x: event.clientX, y: event.clientY } : null;
  }, { capture: true, passive: true });
  listen(document, 'pointermove', event => {
    if (tap?.id === event.pointerId && Math.hypot(event.clientX - tap.x, event.clientY - tap.y) > 10) tap = null;
  }, { capture: true, passive: true });
  listen(document, 'pointerup', event => {
    if (tap?.id !== event.pointerId) return;
    const record = tap.record;
    tap = null;
    if (record.element.contains(event.target)) showAutomatic(record);
  }, { capture: true });
  listen(document, 'keydown', event => {
    for (const record of records.values()) {
      if (record.element.contains(event.target) || event.target.contains(record.element)) record.pointerFocus = false;
    }
  }, { capture: true });
  listen(document, 'pointercancel', () => { tap = null; }, { capture: true });
  // A queued scroll event can follow pointerover after scrollIntoView. Do not
  // cancel that new hover; pointerout owns cancellation when the target leaves.
  listen(document, 'scroll', () => { tap = null; }, { capture: true, passive: true });
  return {
    hideTip, focusTip, syncDescription, showTip,
    dispose(record) {
      hideTip(record);
      if (tap?.record === record) tap = null;
      record.layerAbort?.abort();
      record.ownedTip?.remove();
      const ids = (record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(id => id && id !== `${record.id}-tooltip`);
      if (ids.length) record.element.setAttribute('aria-describedby', ids.join(' '));
      else record.element.removeAttribute('aria-describedby');
    },
    dismissOutside(target) {
      if (visibleTip && !visibleTip.element.contains(target) && !visibleTip.tip?.contains(target)) hideTip(visibleTip);
    },
    escape(event) {
      if (event.key === 'Escape' && visibleTip) { hideTip(visibleTip); event.preventDefault(); event.stopPropagation(); }
    },
    reposition() { if (visibleTip) positionTip(visibleTip); },
    hide() { if (visibleTip) hideTip(visibleTip); },
  };
};
