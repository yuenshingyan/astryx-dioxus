// Astryx ButtonGroup's list-focus/connected-surface bridge. No application work.
// See THIRD_PARTY_NOTICES.md for the pinned upstream contracts.
const id = await dioxus.recv();
const root = document.querySelector(`[data-astryx-button-group-id="${id}"]`);
if (!root) return;
const runtimeKey = Symbol.for('astryx.dioxus.button-group.runtime');
if (!document[runtimeKey]) {
  const groups = new Map();
  const abort = new AbortController();
  const boundary = '[role="group"],[popover],dialog';
  const selector = 'button,a[href],[tabindex]';
  const coarse = matchMedia('(pointer: coarse)');
  const owner = element => element?.closest?.(boundary);
  const isVisible = element => {
    if (!element.isConnected || element.closest('[hidden],[inert]')) return false;
    const css = getComputedStyle(element);
    return css.visibility === 'visible' && css.display !== 'none' && element.getClientRects().length > 0;
  };
  const isEnabled = element => !element.matches('[aria-disabled="true"]');
  const members = group => [...group.querySelectorAll(selector)].filter(element => owner(element) === group && isVisible(element) && !element.matches(':disabled'));
  const stamp = (element, name, value) => {
    if (value == null) { if (element.hasAttribute(name)) element.removeAttribute(name); }
    else if (element.getAttribute(name) !== value) element.setAttribute(name, value);
  };
  const surfaceSlots = element => [...element.children].flatMap(child => {
    if (child.matches('[popover],template,dialog')) return [];
    return getComputedStyle(child).display === 'contents' ? surfaceSlots(child) : [child];
  });
  const keyOf = element => element?.getAttribute('data-astryx-button-id') || element?.id || null;
  const isMenu = element => ['true', 'menu', 'listbox'].includes(element.getAttribute('aria-haspopup'));
  const hitArea = (record, list) => {
    const root = record.root;
    const tracks = coarse.matches && root.getAttribute('aria-disabled') !== 'true';
    const tracked = list.filter(element => element.matches('.astryx-button') && isEnabled(element) && !isMenu(element));
    list.filter(element => element.matches('.astryx-button')).forEach(element => stamp(element, 'data-astryx-group-track', tracks && tracked.includes(element) ? 'true' : null));
    const box = root.getBoundingClientRect();
    let left = 0, right = 0, top = 0, bottom = 0;
    if (tracks && box.width > 0 && box.height > 0) {
      tracked.forEach(element => {
        const rect = element.getBoundingClientRect();
        const x = Math.max(0, (44 - rect.width) / 2), y = Math.max(0, (44 - rect.height) / 2);
        left = Math.max(left, box.left - rect.left + x);
        right = Math.max(right, rect.right + x - box.right);
        top = Math.max(top, box.top - rect.top + y);
        bottom = Math.max(bottom, rect.bottom + y - box.bottom);
      });
      document.querySelectorAll('button,a[href],input,select,textarea,[tabindex]').forEach(element => {
        if (root.contains(element) || element.contains(root) || !isVisible(element)
          || element.closest('[popover]:popover-open,dialog:modal') !== root.closest('[popover]:popover-open,dialog:modal')) return;
        const rect = element.getBoundingClientRect();
        if (rect.top < box.bottom && rect.bottom > box.top) {
          if (rect.right <= box.left) left = Math.min(left, (box.left - rect.right) / 2);
          if (rect.left >= box.right) right = Math.min(right, (rect.left - box.right) / 2);
        }
        if (rect.left < box.right && rect.right > box.left) {
          if (rect.bottom <= box.top) top = Math.min(top, (box.top - rect.bottom) / 2);
          if (rect.top >= box.bottom) bottom = Math.min(bottom, (rect.top - box.bottom) / 2);
        }
      });
    }
    const scaleX = box.width / root.offsetWidth || 1, scaleY = box.height / root.offsetHeight || 1;
    Object.entries({left:left/scaleX,right:right/scaleX,top:top/scaleY,bottom:bottom/scaleY}).forEach(([side, growth]) => {
      const name = `--_group-hit-${side}`;
      const value = `${Math.max(0, growth)}px`;
      if (root.style.getPropertyValue(name) !== value) root.style.setProperty(name, value);
    });
  };
  const repair = record => {
    if (record.isDisposed) return;
    stamp(record.root, 'data-astryx-touch-action', coarse.matches && record.root.getAttribute('aria-disabled') !== 'true'
      ? (record.root.dataset.orientation === 'vertical' ? 'pan-x' : 'pan-y') : null);
    const list = members(record.root);
    const current = new Set(list);
    record.originals.forEach((original, element) => {
      if (!current.has(element)) {
        stamp(element, 'tabindex', original);
        stamp(element, 'data-astryx-group-position', null);
        stamp(element, 'data-astryx-group-orientation', null);
        stamp(element, 'data-astryx-group-track', null);
        record.originals.delete(element);
      }
    });
    list.forEach(element => {
      if (!record.originals.has(element)) record.originals.set(element, element.getAttribute('tabindex'));
    });
    const active = list.includes(document.activeElement) ? document.activeElement : null;
    const replacement = record.stopKey && list.find(element => keyOf(element) === record.stopKey);
    // Keep a disabled/busy holder discoverable; repair removal immediately.
    const stop = active || (list.includes(record.stop) ? record.stop : replacement) || list.find(isEnabled) || list[0];
    list.forEach(element => stamp(element, 'tabindex', element === stop ? '0' : '-1'));
    record.stop = stop || null;
    record.stopKey = keyOf(stop);
    hitArea(record, list);
    list.forEach(element => {
      if (!record.observed.has(element)) { record.resize.observe(element); record.observed.add(element); }
    });
    record.observed.forEach(element => { if (!current.has(element)) { record.resize.unobserve(element); record.observed.delete(element); } });
    const slots = surfaceSlots(record.root);
    const orientation = record.root.dataset.orientation || 'horizontal';
    list.filter(element => element.matches('.astryx-button[data-astryx-grouped="true"]')).forEach(element => {
      const slot = slots.find(candidate => candidate === element || candidate.contains(element));
      const first = slot === slots[0];
      const last = slot === slots.at(-1);
      stamp(element, 'data-astryx-group-position', first && last ? 'only' : first ? 'first' : last ? 'last' : 'middle');
      stamp(element, 'data-astryx-group-orientation', orientation);
    });
  };
  // The group root can commit before reactive children finish their next render.
  // Never let that transition window activate or navigate a grouped Button.
  document.addEventListener('click', event => {
    const group = owner(event.target);
    if (groups.has(group) && group.getAttribute('aria-disabled') === 'true' && event.target.closest?.('.astryx-button')) {
      event.preventDefault(); event.stopImmediatePropagation();
    }
  }, { capture: true, signal: abort.signal });
  // Dioxus's delegated caller handlers run first. Their preventDefault wins.
  document.addEventListener('keydown', event => {
    if (event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey) return;
    const record = groups.get(owner(event.target));
    if (!record) return;
    const list = members(record.root);
    const focused = list.find(element => element === event.target);
    if (!focused) return;
    const enabled = list.filter(isEnabled);
    const candidates = enabled;
    const horizontal = record.root.dataset.orientation !== 'vertical';
    const rtl = getComputedStyle(record.root).direction === 'rtl';
    const next = horizontal ? (rtl ? 'ArrowLeft' : 'ArrowRight') : 'ArrowDown';
    const previous = horizontal ? (rtl ? 'ArrowRight' : 'ArrowLeft') : 'ArrowUp';
    if (!candidates.length) {
      if ([next, previous, 'Home', 'End'].includes(event.key)) event.preventDefault();
      return;
    }
    let destination;
    if (event.key === 'Home') destination = candidates[0];
    else if (event.key === 'End') destination = candidates.at(-1);
    else if (event.key === next || event.key === previous) {
      const direction = event.key === next ? 1 : -1;
      const index = list.indexOf(focused);
      for (let step = 1; step <= list.length; step++) {
        const candidate = list[(index + direction * step + list.length) % list.length];
        if (candidates.includes(candidate)) { destination = candidate; break; }
      }
    }
    if (!destination) return;
    event.preventDefault();
    record.stop = destination;
    record.stopKey = keyOf(destination);
    list.forEach(element => stamp(element, 'tabindex', element === destination ? '0' : '-1'));
    destination.focus();
  }, { signal: abort.signal });
  document.addEventListener('focusin', event => {
    const record = groups.get(owner(event.target));
    if (record && document.activeElement === event.target) repair(record);
  }, { signal: abort.signal });
  document.addEventListener('toggle', event => {
    groups.forEach(record => { if (event.target.contains(record.root)) repair(record); });
  }, { capture: true, signal: abort.signal });
  coarse.addEventListener('change', () => groups.forEach(repair), { signal: abort.signal });
  document[runtimeKey] = {
    groups,
    attach(root) {
      const record = { root, originals: new Map(), observed: new Set(), stop: null, stopKey: null, isDisposed: false };
      record.resize = new ResizeObserver(() => repair(record));
      record.resize.observe(root);
      if (root.parentElement) record.resize.observe(root.parentElement);
      const neighbours = new MutationObserver(() => repair(record));
      if (root.parentElement) neighbours.observe(root.parentElement, { childList: true, subtree: true, attributes: true,
        attributeFilter: ['style', 'class', 'hidden', 'disabled'] });
      const observer = new MutationObserver(() => repair(record));
      groups.set(root, record);
      repair(record);
      observer.observe(root, { childList: true, subtree: true, attributes: true,
        attributeFilter: ['disabled', 'aria-disabled', 'aria-busy', 'hidden', 'style', 'class', 'tabindex', 'role', 'popover', 'data-orientation'] });
      return () => {
        record.isDisposed = true;
        observer.disconnect();
        neighbours.disconnect();
        record.resize.disconnect();
        record.originals.forEach((value, element) => {
          stamp(element, 'tabindex', value);
          stamp(element, 'data-astryx-group-position', null);
          stamp(element, 'data-astryx-group-orientation', null);
          stamp(element, 'data-astryx-group-track', null);
        });
        ['left','right','top','bottom'].forEach(side => root.style.removeProperty(`--_group-hit-${side}`));
        stamp(root, 'data-astryx-touch-action', null);
        groups.delete(root);
        if (!groups.size) { abort.abort(); delete document[runtimeKey]; }
      };
    },
  };
}
const dispose = document[runtimeKey].attach(root);
try {
  while (true) {
    const command = await dioxus.recv();
    if (command === 'dispose' || command == null) break;
  }
} finally { dispose(); }
