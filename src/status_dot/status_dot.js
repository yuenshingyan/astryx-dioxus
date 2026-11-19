// StatusDot owns FR5/AR2 placement, root semantics and host-name delegation.
// Placement is native DOM state, never an inherited replacement context.
const createStatusDotTooltipSupport = ({ records, listen, show, hide }) => {
  const dots = new Set(), names = new Map();
  let observer = null;
  const hostSelector = 'button,a[href],[role~="button"],[role~="link"],[role~="menuitem"],[role~="menuitemcheckbox"],[role~="menuitemradio"],[role~="option"]';
  const normal = text => text.replace(/\s+/g, ' ').trim();
  const isEnabled = record => record.element.isConnected && !record.element.closest('[inert],:disabled,[disabled],[aria-disabled="true"]') && !record.isDisabled;
  const contentName = (node, seen = new Set(), isReference = false) => {
    if (node.nodeType === Node.TEXT_NODE) return node.textContent;
    if (!(node instanceof Element) || seen.has(node) || (!isReference && node.getAttribute('aria-hidden') === 'true') || node.matches('[role="tooltip"],[data-astryx-status-host-name]')) return '';
    const css = getComputedStyle(node);
    if (!isReference && (node.hidden || css.display === 'none' || css.visibility === 'hidden')) return '';
    seen.add(node);
    if (node.hasAttribute('aria-labelledby')) {
      return (node.getAttribute('aria-labelledby') || '').split(/\s+/).map(id => {
        const label = document.getElementById(id);
        return label ? contentName(label, seen, true) : '';
      }).join(' ');
    }
    if (node.hasAttribute('aria-label')) return ` ${node.getAttribute('aria-label')} `;
    if (node.matches('img')) return ` ${node.getAttribute('alt') || ''} `;
    return [...node.childNodes].map(child => contentName(child, seen, isReference)).join('');
  };
  const releaseName = record => {
    const held = record.statusDot.name;
    if (!held) return;
    held.dots.delete(record);
    if (!held.dots.size) {
      held.node.remove();
      const byText = names.get(held.host);
      byText?.delete(held.text);
      if (!byText?.size) names.delete(held.host);
    }
    record.statusDot.name = null;
  };
  const syncName = record => {
    const state = record.statusDot, host = state.host, text = normal(record.tooltip || '');
    const hasOwnName = host?.hasAttribute('aria-label') || host?.hasAttribute('aria-labelledby');
    const alreadyNamed = host && ` ${normal(contentName(host))} `.includes(` ${text} `);
    if (state.name && (state.name.host !== host || state.name.text !== text || hasOwnName || alreadyNamed)) releaseName(record);
    if (!host || !text || hasOwnName || alreadyNamed) return;
    let byText = names.get(host);
    if (!byText) { byText = new Map(); names.set(host, byText); }
    let held = byText.get(text);
    if (!held) {
      const node = document.createElement('span');
      node.setAttribute('data-astryx-status-host-name', '');
      // It must be a host child, not text inside role=img (ignored by AX).
      node.style.cssText = 'position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:pre;border:0';
      node.textContent = ` ${text}`;
      held = { host, text, node, dots: new Set() };
      byText.set(text, held);
    }
    if (held.node.parentElement !== host) host.append(held.node);
    held.dots.add(record); state.name = held;
  };
  const syncSurface = record => {
    const state = record.statusDot;
    if (!state) return;
    const isStandalone = !state.host;
    state.attr('aria-expanded', isStandalone ? String(!!record.isPinned) : null);
    state.attr('aria-controls', isStandalone && record.tip ? `${record.id}-tooltip` : null);
    const name = record.element.getAttribute('aria-label');
    state.attr('aria-describedby', isStandalone && name?.trim() !== record.tooltip?.trim() ? `${record.id}-tooltip` : null);
    state.attr('data-astryx-tooltip-pinned', String(!!record.isPinned));
  };
  const sync = record => {
    const state = record.statusDot, element = record.element;
    if (!state) return;
    const host = element.parentElement?.closest(hostSelector) || null;
    const isStandalone = !host;
    if (host !== state.host) { hide(record); releaseName(record); state.host = host; }
    if (!element.isConnected || !isEnabled(record)) { record.spaceArmed = false; hide(record); }
    state.attr('role', isStandalone ? 'button' : 'img');
    state.attr('tabindex', isStandalone ? '0' : null);
    state.attr('data-astryx-tooltip-placement', isStandalone ? 'standalone' : 'nested');
    if (state.isStandalone !== isStandalone) { state.isStandalone = isStandalone; record.reportPlacement?.(isStandalone); }
    const shouldWatch = isStandalone && isEnabled(record);
    if (shouldWatch && !state.releaseHit) state.releaseHit = record.holdHitArea(element);
    else if (!shouldWatch && state.releaseHit) { state.releaseHit(); state.releaseHit = null; }
    state.attr('data-astryx-hit-area', shouldWatch ? 'true' : null);
    syncName(record); syncSurface(record);
  };
  const ensureObserver = () => {
    if (observer) return;
    observer = new MutationObserver(() => dots.forEach(sync));
    observer.observe(document.documentElement, { subtree: true, childList: true, characterData: true, attributes: true, attributeFilter: ['role', 'href', 'tabindex', 'aria-label', 'aria-labelledby', 'aria-describedby', 'aria-controls', 'aria-expanded', 'aria-hidden', 'hidden', 'class', 'style', 'disabled', 'aria-disabled', 'inert'] });
  };
  listen(document, 'click', event => {
    for (const record of dots) {
      if (!record.element.contains(event.target)) continue;
      const action = event.target.closest?.('button,a[href],input,select,textarea,[contenteditable=""],[contenteditable="true"],[role="button"],[role="link"]');
      record.isOwnClick = !action || action === record.element || action.contains(record.element);
    }
  }, { capture: true });
  return {
    onHide(record) { record.statusDot?.onHide(); },
    onShow(record) { record.statusDot?.onShow(); },
    attach(record) {
      const owned = new Map(), external = new Set();
      let callerNames = new Set((record.callerAttributeNames || []).map(name => name.toLowerCase()));
      const state = {
        host: null, name: null, isStandalone: null, releaseHit: null,
        attr(name, value) {
          // Explicit caller props win, including values equal to our defaults.
          if ((record.callerAttributeNames || []).some(caller => caller.toLowerCase() === name)) { owned.delete(name); return; }
          if (external.has(name)) return;
          const element = record.element, current = element.getAttribute(name);
          let entry = owned.get(name);
          // Native caller mutations win as well as Rust props. Never keep
          // fighting a caller's role, tab stop, or ARIA in the observer.
          if (entry && current !== entry.value) { owned.delete(name); external.add(name); return; }
          if (!entry && current !== null && (name === 'tabindex' || name.startsWith('aria-') || (name === 'role' && current !== 'img'))) { external.add(name); return; }
          if (value === null) {
            if (entry && current === entry.value) {
              if (entry.original === null) element.removeAttribute(name);
              else element.setAttribute(name, entry.original);
            }
            owned.delete(name); return;
          }
          if (!entry) { entry = { original: current, value }; owned.set(name, entry); }
          entry.value = value;
          if (current !== value) element.setAttribute(name, value);
        },
        syncCallerNames() {
          const next = new Set((record.callerAttributeNames || []).map(name => name.toLowerCase()));
          for (const name of callerNames) if (!next.has(name)) external.delete(name);
          callerNames = next;
        },
        onHide() { record.isPinned = false; record.spaceArmed = false; syncSurface(record); },
        onShow() { syncSurface(record); },
      };
      record.statusDot = state; dots.add(record); ensureObserver(); sync(record);
    },
    update(record) { record.statusDot?.syncCallerNames(); sync(record); },
    isHoverOnly(record) { return !!record.statusDot?.host; },
    isEnabled,
    command(record, command) {
      sync(record);
      if (!record.statusDot || record.statusDot.host || !isEnabled(record)) return;
      if (command === 'space-cancel') { record.spaceArmed = false; return; }
      if (command.startsWith('space-') || command === 'enter') {
        if (document.activeElement !== record.element) return;
        if (command === 'space-down') { record.spaceArmed = true; return; }
        if (command === 'space-up') { if (!record.spaceArmed) return; record.spaceArmed = false; }
      } else if (command !== 'toggle' || !record.isOwnClick) return;
      if (record.isPinned) { hide(record); return; }
      show(record);
    },
    dispose(record) {
      const state = record.statusDot;
      if (!state) return;
      releaseName(record); state.releaseHit?.();
      for (const name of ['role', 'tabindex', 'aria-expanded', 'aria-controls', 'aria-describedby', 'data-astryx-tooltip-placement', 'data-astryx-tooltip-pinned', 'data-astryx-hit-area']) state.attr(name, null);
      dots.delete(record); record.statusDot = null;
      if (!dots.size) { observer?.disconnect(); observer = null; }
    },
  };
};
