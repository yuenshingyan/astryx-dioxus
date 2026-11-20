// Adapted from Astryx usePressFeedback/pressFeedback. See THIRD_PARTY_NOTICES.md.
// Shared press lifetime and gesture routing; Tooltip presentation stays with its owner.
// Button-only hit geometry never applies to text links.
const runtimeKey = Symbol.for('astryx.dioxus.button.runtime');
const installPressFeedback = () => {
if (!document[runtimeKey]) {
  // The shared release animates a numeric strength, never the card's layout.
  if (CSS.registerProperty) {
    try { CSS.registerProperty({ name: '--_astryx-press-strength', syntax: '<number>', inherits: true, initialValue: '0' }); }
    catch { /* A previous controller lifetime already registered the property. */ }
  }
  const records = new Map();
  const listeners = new AbortController();
  const signal = listeners.signal;
  const coarse = matchMedia('(pointer: coarse)');
  const reduced = matchMedia('(prefers-reduced-motion: reduce)');
  let gesture = null;
  let keyboardPress = null;
  let suppressed = null;
  let lastScroll = -Infinity;
  const enabled = element => element && !element.matches(':disabled,[aria-disabled="true"],[data-disabled="true"]');
  const pressRoot = '.astryx-button,.astryx-link[data-astryx-feedback-id],[data-astryx-container-press="true"]';
  const interactiveSelectors = 'button,a,input,select,textarea,[role="button"],[role="link"],[role="checkbox"],[role="radio"],[role="switch"],[role="tab"],[role="menuitem"],[role="option"],[role="combobox"],[role="listbox"],[role="slider"],[role="spinbutton"],[data-pressable-container]';
  const isContainer = element => element?.hasAttribute('data-astryx-container-press');
  const nestedContainerTarget = (target, root) => {
    const ownAction = root.querySelector('[data-astryx-container-action]');
    if (ownAction?.contains(target)) return false;
    for (let node = target; node && node !== root && node !== document.body; node = node.parentElement) {
      if ((node.matches?.(interactiveSelectors) && node.getAttribute('aria-readonly') !== 'true')
        || isLayerSurface(node) || isLayerRoot(node)) return true;
    }
    return false;
  };
  const isButton = element => element.matches('.astryx-button[data-astryx-button-id]');
  const isTracked = element => isButton(element) && element.dataset.astryxGroupTrack === 'true';
  const groupOwner = element => {
    const boundary = element?.closest('[role="group"],[popover],dialog');
    return boundary?.hasAttribute('data-astryx-button-group') ? boundary : null;
  };
  const isMenu = element => ['true', 'menu', 'listbox'].includes(element.getAttribute('aria-haspopup'));
  const listen = (target, name, handler, options = {}) => target.addEventListener(name, handler, { ...options, signal });
  const exitTiming = element => readThemeTimingTokens(element, '--duration-exit', '--ease-exit', {
    duration: '175ms', milliseconds: 175, missingDuration: 'var(--duration-fast, 175ms)', easing: 'cubic-bezier(0.3, 0, 0.6, 0.6)',
  });
  const clearTimers = record => {
    record.timers.forEach(clearTimeout); record.timers.clear();
    if (record.frame) cancelAnimationFrame(record.frame);
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
      if (isContainer(record.element)) {
        if (state === 'on') record.element.style.setProperty('--_astryx-press-strength', '1');
        if (!state) record.element.style.removeProperty('--_astryx-press-strength');
      }
    }
  };
  const release = record => {
    if (!record) return;
    const epoch = record.pressEpoch;
    press(record, 'fading');
    const timing = exitTiming(record.element);
    const canFadeReduced = isContainer(record.element) || record.pressPointerType === 'touch' || coarse.matches;
    const duration = reduced.matches ? (canFadeReduced ? Math.min(200, timing.duration) : 0) : timing.duration;
    later(record, () => {
      const finish = () => { if (record.pressEpoch === epoch) press(record, null); };
      // CSS owns the release, including live theme/tempo updates. A busy main
      // thread can delay its first frame beyond the clock; nested press owners
      // and infinite content animations must not extend this surface's release.
      const animations = isContainer(record.element) && record.element.getAnimations
        ? record.element.getAnimations({ subtree: true }).filter(animation => {
          const effect = animation.effect, target = effect?.target;
          return target && 'animationName' in animation
            && (target === record.element || target.closest(pressRoot) === record.element)
            && Number.isFinite(Number(effect.getComputedTiming().endTime)) && animation.playState !== 'finished';
        }) : [];
      if (animations.length) Promise.allSettled(animations.map(animation => animation.finished)).then(finish);
      else finish();
    }, duration);
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
    const direct = hit?.closest(pressRoot);
    const layer = hit?.closest('dialog:modal,[popover]:popover-open') || null;
    if (records.has(direct)) {
      // Text links use the browser's actual hit only, never Button hit growth
      // or per-link geometry measurement.
      if (!isButton(direct)) return group || (isContainer(direct) && nestedContainerTarget(hit, direct)) ? null : records.get(direct);
      const box = direct.getBoundingClientRect();
      if (x >= box.left && x <= box.right && y >= box.top && y <= box.bottom) {
        return !group || (isTracked(direct) && groupOwner(direct) === group && !isMenu(direct)) ? records.get(direct) : null;
      }
    }
    // A Tooltip-owned span/band is already a native target, not a candidate
    // for Button's legacy geometric growth. Nested dots reached the host above.
    const statusDot = hit?.closest('[data-astryx-status-tooltip-id]');
    if (records.get(statusDot)?.isStatusDot) return null;
    const foreign = hit?.closest('button,a[href],input,select,textarea,[role="button"],[role="link"],[role="menuitem"],[role="option"],[tabindex]');
    // Tooltip-only toggles can share this record map without belonging to the
    // press router. Their native hit must never be redirected to a nearby Button.
    if (foreign && (!records.has(foreign) || !foreign.matches(pressRoot))) return null;
    // A touch band grows toward 44px, stopping at the midpoint to neighbours.
    return [...records.values()].find(record => {
      const element = record.element;
      if (!isButton(element) || !enabled(element) || (group && (!isTracked(element) || groupOwner(element) !== group || isMenu(element)))) return false;
      if (element.closest('[inert]') || (element.closest('dialog:modal,[popover]:popover-open') || null) !== layer) return false;
      const host = groupOwner(element);
      if (host && element.dataset.astryxGroupTrack === 'true') {
        const bounds = host.getBoundingClientRect();
        const scaleX = bounds.width / host.offsetWidth || 1, scaleY = bounds.height / host.offsetHeight || 1;
        const growth = side => parseFloat(host.style.getPropertyValue(`--_group-hit-${side}`)) || 0;
        if (x < bounds.left - growth('left') * scaleX || x > bounds.right + growth('right') * scaleX
          || y < bounds.top - growth('top') * scaleY || y > bounds.bottom + growth('bottom') * scaleY) return false;
      }
      const css = getComputedStyle(element);
      if (css.visibility !== 'visible' || css.pointerEvents === 'none') return false;
      const box = element.getBoundingClientRect();
      if (box.width <= 0 || box.height <= 0) return false;
      const dx = Math.max(0, (44 - box.width) / 2);
      const dy = Math.max(0, (44 - box.height) / 2);
      if (x < box.left - dx || x > box.right + dx || y < box.top - dy || y > box.bottom + dy) return false;
      const distance = Math.hypot(x - (box.left + box.width / 2), y - (box.top + box.height / 2));
      return ![...records.keys()].some(other => {
        if (other === element || !isButton(other) || !enabled(other)) return false;
        const rect = other.getBoundingClientRect();
        if (rect.width <= 0 || rect.height <= 0 || other.closest('[inert]') || (other.closest('dialog:modal,[popover]:popover-open') || null) !== layer) return false;
        return Math.hypot(x - (rect.left + rect.width / 2), y - (rect.top + rect.height / 2)) < distance;
      });
    });
  };
  const cancelKeyboardPress = () => {
    if (keyboardPress) press(keyboardPress.record, null);
    keyboardPress = null;
  };
  const cancelNestedContainerPresses = target => {
    const visited = new Set();
    for (let source = target; source && !visited.has(source);) {
      visited.add(source);
      for (let root = source.closest?.('[data-astryx-container-press]'); root; root = root.parentElement?.closest('[data-astryx-container-press]')) {
        if (!nestedContainerTarget(target, root)) continue;
        const record = records.get(root);
        if (gesture && (gesture.original === record || gesture.current === record)) {
          cancelTimer(gesture.original, gesture.timer); press(gesture.current, null); gesture = null;
        }
        if (keyboardPress?.record === record) cancelKeyboardPress();
        press(record, null);
      }
      const layer = closestLayerRoot(source);
      source = layer ? layerSurfaceOwner(layer) : null;
    }
  };
  const tips = createTooltipController({ records, listen, later, cancelTimer });
  const { hideTip, focusTip, syncDescription, showTip } = tips;
  listen(document, 'keydown', event => {
    tips.escape(event);
    if (event.isComposing || event.keyCode === 229) return;
    const actionRoot = event.target.matches?.('[data-astryx-container-action]') ? event.target.closest('[data-astryx-container-press]') : null;
    const isOwnAction = actionRoot?.querySelector('[data-astryx-container-action]') === event.target;
    const record = records.get(event.target) || (isOwnAction ? records.get(actionRoot) : null);
    if (record) {
      record.pointerFocus = false;
      const isActionKey = isOwnAction
        ? event.key === 'Enter' || (event.key === ' ' && event.target.tagName !== 'A')
        : event.key === ' ' && record.element.tagName === 'BUTTON';
      if ((coarse.matches || isContainer(record.element)) && isActionKey && enabled(record.element)) {
        if (keyboardPress?.record !== record) cancelKeyboardPress();
        keyboardPress = { record, target: event.target, key: event.key };
        press(record, 'on');
      }
    }
  }, { capture: true });
  listen(document, 'keyup', event => {
    if (keyboardPress && event.key === keyboardPress.key) {
      const record = keyboardPress.record;
      keyboardPress = null;
      release(record);
    }
  }, { capture: true });
  listen(document, 'focusout', event => {
    if (keyboardPress?.target === event.target && event.relatedTarget !== event.target) cancelKeyboardPress();
  }, { capture: true });
  listen(document, 'pointerdown', event => {
    if (!event.isPrimary || event.button !== 0) return;
    suppressed = null;
    cancelNestedContainerPresses(event.target);
    tips.dismissOutside(event.target);
    const record = event.pointerType === 'touch'
      ? recordAt(event.clientX, event.clientY)
      : records.get(event.target.closest?.(pressRoot));
    if (!record) return;
    if (isContainer(record.element) && nestedContainerTarget(event.target, record.element)) {
      press(record, null);
      return;
    }
    record.pointerFocus = true;
    record.pressPointerType = event.pointerType;
    if (!record.isPinned) hideTip(record);
    if (!enabled(record.element)) {
      if (event.pointerType === 'touch' && record.tooltip) {
        gesture = { id: event.pointerId, original: record, current: record, nativeTarget: event.target.closest?.(pressRoot) || event.target, x: event.clientX, y: event.clientY, isHintOnly: true, cancelled: false };
      }
      return;
    }
    if (event.pointerType !== 'touch' && !coarse.matches && !isContainer(record.element)) return;
    if (gesture) { cancelTimer(gesture.original, gesture.timer); press(gesture.current, null); }
    const owner = isTracked(record.element) ? groupOwner(record.element) : null;
    const group = coarse.matches && ['touch', 'pen'].includes(event.pointerType) && owner?.getAttribute('aria-disabled') !== 'true' && !isMenu(record.element) ? owner : null;
    gesture = { id: event.pointerId, original: record, current: record, nativeTarget: event.target.closest?.(pressRoot) || event.target, group, x: event.clientX, y: event.clientY, scrolling: isScrolling(record.element), cancelled: false };
    if (event.pointerType === 'touch' && !group && gesture.scrolling && performance.now() - lastScroll < 150) {
      gesture.cancelled = true;
      return;
    }
    if (event.pointerType === 'touch' && !group && gesture.scrolling) {
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
    // Row/card presses use the source pan hysteresis even outside scrollers.
    // Momentary Button tracking keeps its existing independent profile.
    if ((gesture.scrolling || isContainer(gesture.original.element)) && (gesture.group ? crossMovement : Math.hypot(dx, dy)) > 10) {
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
    suppressed = { element: gesture.group || gesture.nativeTarget, pointerId: gesture.id, until: performance.now() + 500 };
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
    if (finished.cancelled || (!finished.current && !finished.group)) {
      press(finished.current, null);
      suppressed = { element: finished.group || finished.nativeTarget, pointerId: finished.id, until: performance.now() + 500 };
      return;
    }
    if (finished.group) {
      const candidate = recordAt(event.clientX, event.clientY, finished.group);
      if (candidate !== finished.current) { press(finished.current, null); finished.current = candidate; }
    }
    const current = finished.current;
    if (!current || !current.element.isConnected || !enabled(current.element)) {
      press(current, null);
      suppressed = { element: finished.group || finished.nativeTarget, pointerId: finished.id, until: performance.now() + 500 };
      return;
    }
    if (!current.element.hasAttribute('data-astryx-press')) {
      press(current, 'on');
      const epoch = current.pressEpoch;
      later(current, () => { if (current.pressEpoch === epoch) release(current); }, 100);
    } else release(current);
    if (finished.group) {
      // Native touch click targets the touch-down member. The group activates
      // the release member instead, once, and suppresses only the native tail.
      current.element.click();
      suppressed = { element: finished.group, pointerId: finished.id, until: performance.now() + 500 };
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
  listen(document, 'scroll', () => { lastScroll = performance.now(); if (gesture) cancelGesture(); tips.reposition(); }, { capture: true, passive: true });
  listen(window, 'resize', () => { tips.reposition(); }, { passive: true });
  listen(window, 'blur', () => { cancelGesture(); cancelKeyboardPress(); tips.hide(); });
  listen(document, 'visibilitychange', () => { if (document.hidden) { cancelGesture(); cancelKeyboardPress(); tips.hide(); } });
  document[runtimeKey] = {
    records,
    cancelPress(element) { press(records.get(element), null); },
    attach(element, config) {
      const record = { element, timers: new Set(), isFocused: document.activeElement === element || (config.isTruncation && document.activeElement?.contains(element)), ...config };
      records.set(element, record);
      tips.attach(record);
      syncDescription(record);
      if (record.isFocused) focusTip(record);
      return {
        show() { tips.show(record); },
        preview() { tips.showTip(record, record.tooltip); },
        hide() { tips.hideTip(record); },
        command(command) { tips.command(record, command); },
        update(config) {
          Object.assign(record, config);
          tips.update(record);
          syncDescription(record);
          if (!enabled(element)) { press(record, null); if (gesture?.current === record) cancelGesture(); if (keyboardPress?.record === record) cancelKeyboardPress(); }
          if (record.tip && record.tooltipPolicy) tips.refreshPolicyTip(record);
          else if (record.tip) { hideTip(record); if (record.isFocused) focusTip(record); }
        },
        dispose() {
          if (keyboardPress?.record === record) cancelKeyboardPress();
          if (gesture?.original === record || gesture?.current === record) cancelGesture();
          tips.dispose(record); clearTimers(record); press(record, null); records.delete(element);
          if ((record.isFocused || record.hadFocus) && (!document.activeElement || document.activeElement === document.body || document.activeElement === element)) {
            const replacement = document.querySelector(`[data-astryx-button-id="${CSS.escape(record.id)}"],[data-astryx-feedback-id="${CSS.escape(record.id)}"]`);
            if (replacement && replacement !== element) replacement.focus({ preventScroll: true });
          }
          if (!records.size) { listeners.abort(); delete document[runtimeKey]; }
        },
      };
    },
  };
}
};
