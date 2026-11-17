// Adapted from Astryx's Button/Tooltip/press contracts. See THIRD_PARTY_NOTICES.md.
// This renderer bridge owns only DOM measurement, native layers, and pointer feedback.
const initial = await dioxus.recv();
const runtimeKey = Symbol.for('astryx.dioxus.button.runtime');
if (!document[runtimeKey]) {
  const records = new Map();
  const listeners = new AbortController();
  const signal = listeners.signal;
  const coarse = matchMedia('(pointer: coarse)');
  const reduced = matchMedia('(prefers-reduced-motion: reduce)');
  let gesture = null;
  let suppressed = null;
  let visibleTip = null;
  let lastScroll = -Infinity;
  const enabled = element => element && !element.matches(':disabled,[aria-disabled="true"]');
  const groupMembers = group => [...group.querySelectorAll('.astryx-button')]
    .filter(element => element.closest('[data-astryx-button-group]') === group);
  const listen = (target, name, handler, options = {}) => target.addEventListener(name, handler, { ...options, signal });
  const milliseconds = value => value.trim().endsWith('ms') ? parseFloat(value) : parseFloat(value) * 1000;
  const exitDuration = element => milliseconds(getComputedStyle(element).getPropertyValue('--duration-exit') || '175ms');
  const clearTimers = record => {
    record.timers.forEach(clearTimeout); record.timers.clear();
    if (record.frame) cancelAnimationFrame(record.frame);
  };
  const synchronizeRotors = record => {
    if (!record.element.querySelector('[data-astryx-spinner-rotor]')) return;
    if (record.frame) cancelAnimationFrame(record.frame);
    record.frame = requestAnimationFrame(() => {
      record.frame = null;
      record.element.querySelectorAll('[data-astryx-spinner-rotor]').forEach(rotor => {
        rotor.getAnimations().forEach(animation => { animation.startTime = 0; });
      });
    });
  };
  const cancelTimer = (record, timer) => { clearTimeout(timer); record.timers.delete(timer); };
  const later = (record, fn, ms) => {
    const timer = setTimeout(() => { record.timers.delete(timer); fn(); }, ms);
    record.timers.add(timer);
    return timer;
  };
  const press = (record, state) => {
    if (record && records.has(record.element)) {
      if (state === 'on' || !state) record.pressEpoch = (record.pressEpoch || 0) + 1;
      if (state && enabled(record.element)) record.element.dataset.astryxPress = state;
      else delete record.element.dataset.astryxPress;
    }
  };
  const release = record => {
    if (!record) return;
    const epoch = record.pressEpoch;
    press(record, 'fading');
    later(record, () => { if (record.pressEpoch === epoch) press(record, null); }, reduced.matches ? (coarse.matches ? 200 : 0) : exitDuration(record.element));
  };
  const isScrolling = element => {
    for (let parent = element.parentElement; parent; parent = parent.parentElement) {
      const css = getComputedStyle(parent);
      if ((/(auto|scroll|overlay)/.test(css.overflowY) && parent.scrollHeight > parent.clientHeight)
        || (/(auto|scroll|overlay)/.test(css.overflowX) && parent.scrollWidth > parent.clientWidth)) return true;
    }
    return false;
  };
  const recordAt = (x, y, group = null) => {
    const hit = document.elementFromPoint(x, y);
    const direct = hit?.closest('.astryx-button');
    const layer = hit?.closest('dialog:modal,[popover]:popover-open') || null;
    if (records.has(direct) && (!group || direct.closest('[data-astryx-button-group]') === group)) {
      const box = direct.getBoundingClientRect();
      if (x >= box.left && x <= box.right && y >= box.top && y <= box.bottom) return records.get(direct);
    }
    // A touch band grows toward 44px, stopping at the midpoint to neighbours.
    return [...records.values()].find(record => {
      const element = record.element;
      if (!enabled(element) || (group && element.closest('[data-astryx-button-group]') !== group)) return false;
      if (element.closest('[inert]') || (element.closest('dialog:modal,[popover]:popover-open') || null) !== layer) return false;
      const css = getComputedStyle(element);
      if (css.visibility !== 'visible' || css.pointerEvents === 'none') return false;
      const box = element.getBoundingClientRect();
      if (box.width <= 0 || box.height <= 0) return false;
      const dx = Math.max(0, (44 - box.width) / 2);
      const dy = Math.max(0, (44 - box.height) / 2);
      if (x < box.left - dx || x > box.right + dx || y < box.top - dy || y > box.bottom + dy) return false;
      const distance = Math.hypot(x - (box.left + box.width / 2), y - (box.top + box.height / 2));
      return ![...records.keys()].some(other => {
        if (other === element || !enabled(other)) return false;
        const rect = other.getBoundingClientRect();
        if (rect.width <= 0 || rect.height <= 0 || other.closest('[inert]') || (other.closest('dialog:modal,[popover]:popover-open') || null) !== layer) return false;
        return Math.hypot(x - (rect.left + rect.width / 2), y - (rect.top + rect.height / 2)) < distance;
      });
    });
  };
  const hideTip = record => {
    if (!record) return;
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
    const content = element.querySelector('.astryx-button__content')?.cloneNode(true);
    content?.querySelectorAll('[aria-hidden="true"]').forEach(node => node.remove());
    return content?.textContent || '';
  };
  const syncDescription = record => {
    if (!record.tooltip) return;
    const id = `${record.id}-tooltip`;
    const ids = new Set((record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean));
    const name = accessibleName(record.element);
    if (name !== null && name.trim() === record.tooltip.trim()) ids.delete(id);
    else ids.add(id);
    if (ids.size) record.element.setAttribute('aria-describedby', [...ids].join(' '));
    else record.element.removeAttribute('aria-describedby');
  };
  const callerTooltip = record => (record.element.getAttribute('aria-describedby') || '').split(/\s+/)
    .some(id => id && document.getElementById(id)?.getAttribute('role') === 'tooltip' && id !== record.tip?.id);
  const showTip = (record, text, automatic = false) => {
    if (!text || !record.element.isConnected) return;
    if (visibleTip && visibleTip !== record) hideTip(visibleTip);
    hideTip(record);
    const existing = !automatic && document.getElementById(`${record.id}-tooltip`);
    const tip = existing || document.createElement('div');
    tip.id = `${record.id}-tooltip`;
    tip.className = 'astryx-button-tooltip';
    tip.setAttribute('role', 'tooltip');
    tip.setAttribute('popover', 'manual');
    tip.textContent = text;
    // Copy inherited theme/typography onto the top-layer surface, not body defaults.
    const css = getComputedStyle(record.element);
    ['--color-background-inverted', '--color-background-surface', '--radius-container', '--text-label-size', '--text-label-leading'].forEach(name => tip.style.setProperty(name, css.getPropertyValue(name)));
    tip.style.fontFamily = css.fontFamily;
    if (!existing) record.element.after(tip);
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
    if (record.boundTip !== tip) {
      record.layerAbort?.abort();
      record.layerAbort = new AbortController();
      record.boundTip = tip;
      tip.addEventListener('pointerenter', () => cancelTimer(record, record.hideTimer), { signal: record.layerAbort.signal });
      tip.addEventListener('pointerleave', () => { record.hideTimer = later(record, () => hideTip(record), 100); }, { signal: record.layerAbort.signal });
    }
    visibleTip = record;
  };
  const focusTip = record => {
    if (record.pointerFocus) return;
    if (record.tooltip) { showTip(record, record.tooltip); return; }
    if (record.isIconOnly || !record.element.matches(':focus-visible') || callerTooltip(record)) return;
    const label = record.element.querySelector('.astryx-button__label');
    if (label && getComputedStyle(label).visibility !== 'hidden' && label.scrollWidth > label.clientWidth) {
      showTip(record, record.label, true);
    }
  };
  listen(document, 'focusin', event => {
    const record = records.get(event.target);
    if (record) { record.isFocused = true; focusTip(record); }
  }, { capture: true });
  listen(document, 'focusout', event => {
    const record = records.get(event.target);
    if (record) {
      record.hadFocus = record.isFocused;
      record.isFocused = false; record.pointerFocus = false; hideTip(record);
      queueMicrotask(() => { if (record.element.isConnected) record.hadFocus = false; });
    }
  }, { capture: true });
  listen(document, 'pointerover', event => {
    const record = records.get(event.target.closest?.('.astryx-button'));
    if (!record || record.element.contains(event.relatedTarget) || event.pointerType === 'touch' || !record.tooltip) return;
    cancelTimer(record, record.hideTimer);
    record.showTimer = later(record, () => showTip(record, record.tooltip), 200);
  }, { capture: true });
  listen(document, 'pointerout', event => {
    const record = records.get(event.target.closest?.('.astryx-button'));
    if (!record || record.element.contains(event.relatedTarget)) return;
    cancelTimer(record, record.showTimer);
    if (record.tip && !record.isAutomatic && document.activeElement !== record.element) record.hideTimer = later(record, () => hideTip(record), 100);
  }, { capture: true });
  listen(document, 'keydown', event => {
    if (event.key === 'Escape' && visibleTip) { hideTip(visibleTip); event.preventDefault(); event.stopPropagation(); }
    const record = records.get(event.target);
    if (record) {
      record.pointerFocus = false;
      if (coarse.matches && event.key === ' ' && enabled(record.element) && record.element.tagName === 'BUTTON') press(record, 'on');
      const group = record.element.closest('[data-astryx-button-group]');
      if (group) {
        const members = groupMembers(group).filter(button => records.has(button));
        const index = members.indexOf(record.element);
        const horizontal = group.dataset.orientation !== 'vertical';
        const rtl = getComputedStyle(group).direction === 'rtl';
        const next = horizontal ? (rtl ? 'ArrowLeft' : 'ArrowRight') : 'ArrowDown';
        const previous = horizontal ? (rtl ? 'ArrowRight' : 'ArrowLeft') : 'ArrowUp';
        let destination = null;
        if (event.key === next) destination = (index + 1) % members.length;
        if (event.key === previous) destination = (index - 1 + members.length) % members.length;
        if (event.key === 'Home') destination = 0;
        if (event.key === 'End') destination = members.length - 1;
        if (destination !== null) {
          event.preventDefault();
          members.forEach((member, i) => member.tabIndex = i === destination ? 0 : -1);
          members[destination].focus();
        }
      }
    }
  }, { capture: true });
  listen(document, 'keyup', event => { if (coarse.matches && event.key === ' ') release(records.get(event.target)); }, { capture: true });
  listen(document, 'pointerdown', event => {
    if (!event.isPrimary || event.button !== 0) return;
    suppressed = null;
    if (visibleTip && !visibleTip.element.contains(event.target) && !visibleTip.tip?.contains(event.target)) hideTip(visibleTip);
    const record = event.pointerType === 'touch'
      ? recordAt(event.clientX, event.clientY)
      : records.get(event.target.closest?.('.astryx-button'));
    if (!record) return;
    record.pointerFocus = true;
    hideTip(record);
    if (!enabled(record.element)) {
      if (event.pointerType === 'touch' && record.tooltip) {
        gesture = { id: event.pointerId, original: record, current: record, nativeTarget: event.target.closest?.('.astryx-button') || event.target, x: event.clientX, y: event.clientY, isHintOnly: true, cancelled: false };
      }
      return;
    }
    if (event.pointerType !== 'touch' && !coarse.matches) return;
    if (gesture) press(gesture.current, null);
    const group = record.element.closest('[data-astryx-button-group]');
    const isMenu = ['true', 'menu', 'listbox'].includes(record.element.getAttribute('aria-haspopup'));
    gesture = { id: event.pointerId, original: record, current: record, nativeTarget: event.target.closest?.('.astryx-button') || event.target, group: isMenu ? null : group, x: event.clientX, y: event.clientY, scrolling: isScrolling(record.element), cancelled: false };
    if (event.pointerType === 'touch' && gesture.scrolling && performance.now() - lastScroll < 150) {
      gesture.cancelled = true;
      return;
    }
    if (event.pointerType === 'touch' && gesture.scrolling) {
      gesture.timer = later(record, () => { if (gesture?.original === record && !gesture.cancelled) press(gesture.current, 'on'); }, Math.max(0, 150 - (performance.now() - event.timeStamp)));
    } else press(record, 'on');
  }, { capture: true, passive: true });
  listen(document, 'pointermove', event => {
    if (!gesture || gesture.id !== event.pointerId || gesture.cancelled) return;
    const dx = Math.abs(event.clientX - gesture.x);
    const dy = Math.abs(event.clientY - gesture.y);
    if (gesture.isHintOnly) {
      if (Math.hypot(dx, dy) > 10) gesture.cancelled = true;
      return;
    }
    const crossMovement = gesture.group?.dataset.orientation === 'vertical' ? dx : dy;
    if (gesture.scrolling && (gesture.group ? crossMovement : Math.hypot(dx, dy)) > 10) {
      cancelTimer(gesture.original, gesture.timer); press(gesture.current, null); gesture.cancelled = true; return;
    }
    const candidate = recordAt(event.clientX, event.clientY, gesture.group);
    const next = candidate && (gesture.group || candidate === gesture.original) && enabled(candidate.element) ? candidate : null;
    if (next !== gesture.current) { press(gesture.current, null); gesture.current = next; press(next, 'on'); }
    if (gesture.group && (dx > 10 || dy > 10)) event.preventDefault();
  }, { capture: true, passive: false });
  const cancelGesture = () => {
    if (!gesture) return;
    cancelTimer(gesture.original, gesture.timer);
    press(gesture.current, null);
    suppressed = { element: gesture.nativeTarget, pointerId: gesture.id, until: performance.now() + 500 };
    gesture = null;
  };
  listen(document, 'pointercancel', cancelGesture, { capture: true });
  listen(document, 'pointerup', event => {
    if (!gesture || gesture.id !== event.pointerId) return;
    const finished = gesture;
    gesture = null;
    cancelTimer(finished.original, finished.timer);
    if (finished.isHintOnly) {
      if (!finished.cancelled) showTip(finished.original, finished.original.tooltip);
      return;
    }
    if (finished.cancelled || !finished.current) {
      press(finished.current, null);
      suppressed = { element: finished.nativeTarget, pointerId: finished.id, until: performance.now() + 500 };
      return;
    }
    const current = finished.current;
    if (!current.element.hasAttribute('data-astryx-press')) {
      press(current, 'on');
      const epoch = current.pressEpoch;
      later(current, () => { if (current.pressEpoch === epoch) release(current); }, 100);
    } else release(current);
    if (finished.group) {
      // Native touch click targets the touch-down member. The group activates
      // the release member instead, once, and suppresses only the native tail.
      current.element.click();
      suppressed = { element: finished.nativeTarget, pointerId: finished.id, until: performance.now() + 500 };
    } else if (!finished.original.element.contains(event.target)) {
      current.element.click();
      suppressed = { element: finished.nativeTarget, pointerId: finished.id, until: performance.now() + 500 };
    }
  }, { capture: true });
  listen(document, 'click', event => {
    if (suppressed && performance.now() < suppressed.until && suppressed.element.contains(event.target) && event.isTrusted && event.detail > 0
      && (event.pointerId === suppressed.pointerId || event.sourceCapabilities?.firesTouchEvents || !('pointerId' in event))) {
      event.preventDefault(); event.stopImmediatePropagation(); suppressed = null;
    }
  }, { capture: true });
  listen(document, 'scroll', () => { lastScroll = performance.now(); if (gesture) cancelGesture(); if (visibleTip) positionTip(visibleTip); }, { capture: true, passive: true });
  listen(window, 'resize', () => { if (visibleTip) positionTip(visibleTip); }, { passive: true });
  listen(window, 'blur', () => { cancelGesture(); if (visibleTip) hideTip(visibleTip); });
  document[runtimeKey] = {
    records,
    attach(element, config) {
      const record = { element, timers: new Set(), isFocused: document.activeElement === element, ...config };
      records.set(element, record);
      synchronizeRotors(record);
      syncDescription(record);
      if (record.isFocused) focusTip(record);
      const group = element.closest('[data-astryx-button-group]');
      if (group) {
        const members = groupMembers(group);
        element.tabIndex = members.some(member => member !== element && records.has(member) && member.tabIndex === 0) ? -1 : 0;
      }
      return {
        update(config) {
          Object.assign(record, config);
          synchronizeRotors(record);
          syncDescription(record);
          if (!enabled(element)) { press(record, null); if (gesture?.current === record) cancelGesture(); }
          if (record.tip) { hideTip(record); if (record.isFocused) focusTip(record); }
        },
        dispose() {
          if (gesture?.original === record || gesture?.current === record) cancelGesture();
          hideTip(record); clearTimers(record); record.layerAbort?.abort(); records.delete(element);
          if ((record.isFocused || record.hadFocus) && (!document.activeElement || document.activeElement === document.body || document.activeElement === element)) {
            const replacement = document.querySelector(`[data-astryx-button-id="${record.id}"]`);
            if (replacement && replacement !== element) replacement.focus({ preventScroll: true });
          }
          if (group) {
            const members = groupMembers(group).filter(member => records.has(member));
            if (!members.some(member => member.tabIndex === 0)) members[0]?.setAttribute('tabindex', '0');
          }
          if (!records.size) { listeners.abort(); delete document[runtimeKey]; }
        },
      };
    },
  };
}
const [id, label, tooltip, isIconOnly] = initial;
const element = document.querySelector(`[data-astryx-button-id="${id}"]`);
if (!element) return;
const controller = document[runtimeKey].attach(element, { id, label, tooltip, isIconOnly });
try {
  while (true) {
    const update = await dioxus.recv();
    if (update === 'dispose') break;
    const [, label, tooltip, isIconOnly] = update;
    controller.update({ label, tooltip, isIconOnly });
  }
} finally {
  controller.dispose();
}
