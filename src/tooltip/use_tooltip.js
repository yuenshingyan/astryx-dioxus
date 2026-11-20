// Adapted from Astryx Tooltip/useTooltip. See THIRD_PARTY_NOTICES.md.
// Tooltip owns presentation, descriptions and trigger decisions; Text owns first interest.
// Press only notifies dismissal.
const createTooltipController = ({ records, listen, later, cancelTimer }) => {
  const moduleKey = Symbol.for('astryx.dioxus.truncation-presentation.v1');
  const urls = typeof truncationPresentationUrls === 'undefined' ? null : truncationPresentationUrls.join(' ');
  // Explicit-only controllers do not configure or evict the truncation cache.
  // Controllers with new content-hashed URLs must not reuse an older build.
  const cached = urls === null ? null : document[moduleKey];
  const modules = urls === null ? null : cached?.urls === urls ? cached
    : (document[moduleKey] = { urls, loaded: null, pending: null, attempt: 0 });
  const preload = record => {
    if (!modules) return Promise.reject(new Error('Truncation presentation asset URLs were not provided'));
    if (modules.loaded) return Promise.resolve(modules.loaded);
    if (!modules.pending) {
      const suffix = modules.attempt++ ? `?recovery=${modules.attempt}` : '';
      modules.pending = Promise.all(truncationPresentationUrls.map(url => import(url + suffix)))
        .then(([tooltip, layer]) => (modules.loaded = {...tooltip, ...layer}))
        .finally(() => { modules.pending = null; });
    }
    return modules.pending;
  };
  const whenReady = (record, operation, kind) => {
    if (modules?.loaded) { operation(); return; }
    record.pendingPresentation ||= new Map();
    record.pendingPresentation.set(kind, operation);
    const pending = record.pendingPresentation;
    preload(record).then(() => {
      if (record.pendingPresentation !== pending || !record.element.isConnected || records.get(record.element) !== record) return;
      record.pendingPresentation = null;
      for (const action of pending.values()) action();
    }, () => { if (record.pendingPresentation === pending) record.pendingPresentation = null; });
  };
  const forgetPending = record => record.pendingPresentation?.delete('interest');
  let visibleTip = null;
  let isKeyboardModality = true;
  const hideTip = record => {
    if (!record) return;
    forgetPending(record);
    record.isPinned = false;
    record.isTapOpen = false;
    record.pinAnnouncer?.withdrawPending();
    cancelTimer(record, record.showTimer);
    cancelTimer(record, record.hideTimer);
    if (record.tip) {
      clearLayerSurfaceOwner(record.tip);
      const id = record.tip.id;
      if (record.isAutomatic && !record.retainedTip) {
        record.tip.remove();
        record.layerAbort?.abort();
        record.boundTip = null;
        const ids = (record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(value => value && value !== id);
        if (ids.length) record.element.setAttribute('aria-describedby', ids.join(' '));
        else record.element.removeAttribute('aria-describedby');
      } else if (record.tip.matches(':popover-open')) {
        if (record.truncationPresentation) record.truncationPresentation.hide();
        else record.tip.hidePopover();
      }
      // A nested status surface lives outside its interactive host while open,
      // so its visible text does not duplicate the host's hidden name suffix.
      if (record.tipHome) {
        if (record.tip.isConnected && record.tipHome.parentNode) record.tipHome.replaceWith(record.tip);
        else { record.tipHome.remove(); record.tip.remove(); }
        record.tipHome = null;
      }
      record.tip = null;
    }
    if (visibleTip === record) visibleTip = null;
    record.tooltipPolicy?.onHide(record);
  };
  const positionTip = record => {
    if (record.truncationPresentation || !record.tip?.isConnected || !record.element.isConnected) return;
    const box = record.element.getBoundingClientRect();
    const tipBox = record.tip.getBoundingClientRect();
    const viewport = visualViewport;
    const left = viewport?.offsetLeft || 0;
    const top = viewport?.offsetTop || 0;
    const width = viewport?.width || innerWidth;
    const height = viewport?.height || innerHeight;
    const above = box.top - tipBox.height - 8;
    const below = box.bottom + 8;
    const placement = record.isTruncation ? record.element.getAttribute('data-astryx-truncation-placement') || 'above' : 'above';
    let x = box.left + (box.width - tipBox.width) / 2;
    let y = above >= top + 4 ? above : below;
    if (placement === 'below') y = below + tipBox.height <= top + height - 4 ? below : above;
    if (placement === 'start' || placement === 'end') {
      const isRtl = getComputedStyle(record.element).direction === 'rtl';
      const isLeft = (placement === 'start') !== isRtl;
      const before = box.left - tipBox.width - 8;
      const after = box.right + 8;
      x = isLeft ? (before >= left + 4 ? before : after)
        : (after + tipBox.width <= left + width - 4 ? after : before);
      y = box.top + (box.height - tipBox.height) / 2;
    }
    record.tip.style.left = `${Math.max(left + 4, Math.min(x, left + width - tipBox.width - 4))}px`;
    record.tip.style.top = `${Math.max(top + 4, Math.min(y, top + height - tipBox.height - 4))}px`;
  };
  const accessibleName = element => {
    if (element.hasAttribute('aria-labelledby')) return null;
    if (element.hasAttribute('aria-label')) return element.getAttribute('aria-label');
    const content = (element.querySelector('.astryx-button__content') || element).cloneNode(true);
    content?.querySelectorAll('[aria-hidden="true"]').forEach(node => node.remove());
    return content?.textContent || '';
  };
  const syncDescription = record => {
    if (record.tooltipPolicy || record.isTruncation) return;
    const id = `${record.id}-tooltip`;
    const ids = new Set((record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean));
    const name = record.tooltip ? accessibleName(record.element) : null;
    if (!record.tooltip || (name !== null && name.trim() === record.tooltip.trim())) ids.delete(id);
    else ids.add(id);
    if (ids.size) record.element.setAttribute('aria-describedby', [...ids].join(' '));
    else record.element.removeAttribute('aria-describedby');
  };
  const callerTooltip = record => (record.element.getAttribute('aria-describedby') || '').split(/\s+/)
    .some(id => id && document.getElementById(id)?.getAttribute('role') === 'tooltip' && id !== record.tip?.id && id !== record.retainedTip?.id);
  const showTip = (record, text, automatic = false, isClosed = false) => {
    if (!text || !record.element.isConnected || !records.has(record.element)) return;
    if (record.isTruncation && !modules?.loaded) {
      whenReady(record, () => showTip(record, text, automatic, isClosed), isClosed ? 'mount' : 'interest');
      return;
    }
    if (record.isTruncation) text = record.truncationText || text;
    if (record.tooltipPolicy && !record.tooltipPolicy.isEnabled(record)) return;
    if (record.isPinned && record.tip) return;
    if (!isClosed && visibleTip?.isPinned && visibleTip !== record) return;
    if (!isClosed && visibleTip && visibleTip !== record) hideTip(visibleTip);
    if (!isClosed) hideTip(record);
    const existing = record.retainedTip || (!automatic && document.getElementById(`${record.id}-tooltip`));
    const tip = existing || document.createElement('div');
    tip.id = `${record.id}-tooltip`;
    tip.className = record.isTruncation ? 'astryx-tooltip astryx-truncation-tooltip' : 'astryx-tooltip astryx-button-tooltip';
    tip.setAttribute('role', 'tooltip');
    tip.setAttribute('popover', 'manual');
    if (record.truncationPresentation) record.truncationPresentation.update(text);
    else tip.textContent = text;
    // Copy inherited theme/typography onto the top-layer surface, not body defaults.
    const css = getComputedStyle(record.element);
    ['--color-background-inverted', '--color-background-surface', '--radius-container', '--text-label-size', '--text-label-leading'].forEach(name => tip.style.setProperty(name, css.getPropertyValue(name)));
    if (!record.isTruncation) tip.style.fontFamily = css.fontFamily;
    if (!existing) {
      // Automatic Text tips must not become part of an enclosing link's name.
      const host = automatic && record.isTruncation
        ? record.element.closest('a,button,[role="link"],[role="button"]') || record.element
        : record.element;
      if (record.tooltipPolicy) (record.element.closest('dialog:modal') || document.body).append(tip);
      else host.after(tip);
      if (!automatic) record.ownedTip = tip;
    }
    if (record.tooltipPolicy) {
      tip.style.colorScheme = css.colorScheme;
      if (existing && record.ownedTip !== existing && record.tooltipPolicy.isHoverOnly(record) && !automatic) {
        // Only a caller/renderer-owned surface needs its original home restored.
        record.tipHome = document.createComment('astryx-tooltip-home');
        tip.before(record.tipHome);
        (record.element.closest('dialog:modal') || document.body).append(tip);
      }
    }
    if (automatic && record.isTruncation) {
      record.retainedTip = tip;
      record.ownedTip = tip;
      const ids = new Set((record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean));
      if (!ids.has(tip.id)) { ids.add(tip.id); record.ownsTruncationDescription = true; }
      record.element.setAttribute('aria-describedby', [...ids].join(' '));
    }
    if (record.isTruncation && !record.truncationPresentation) {
      record.truncationPresentation = modules.loaded.createTooltipPresentation({record, tip, createLayerPresentation: modules.loaded.createLayerPresentation});
    }
    if (record.tip && isClosed) positionTip(record);
    if (!isClosed) record.tip = tip;
    record.isAutomatic = automatic;
    const name = accessibleName(record.element);
    if (!automatic && !record.tooltipPolicy && (name === null || name.trim() !== text.trim())) {
      const ids = new Set((record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean));
      ids.add(tip.id);
      record.element.setAttribute('aria-describedby', [...ids].join(' '));
    }
    markLayerRoot(tip);
    if (!isClosed) {
      registerLayerSurface(tip, record.element);
      if (record.truncationPresentation) record.truncationPresentation.show();
      else tip.showPopover?.();
      positionTip(record);
    }
    if (record.boundTip !== tip) {
      record.layerAbort?.abort();
      record.layerAbort = new AbortController();
      record.boundTip = tip;
      tip.addEventListener('pointerenter', () => cancelTimer(record, record.hideTimer), { signal: record.layerAbort.signal });
      tip.addEventListener('pointerleave', () => { if (!record.isPinned) record.hideTimer = later(record, () => hideTip(record), 100); }, { signal: record.layerAbort.signal });
      if (record.tooltipPolicy) {
        // A help surface must not activate a label or clickable container.
        tip.addEventListener('click', event => { event.preventDefault(); event.stopPropagation(); }, { signal: record.layerAbort.signal });
      }
    }
    if (!isClosed) {
      visibleTip = record;
      record.tooltipPolicy?.onShow(record);
    }
  };
  const announcePinned = record => {
    // Announcement routing/lifetime belongs to the original shared hook owner,
    // including native-modal isolation and withdrawal on owner removal.
    const announcements = document[Symbol.for('astryx.dioxus.use-announce.v2')];
    if (announcements) {
      record.pinAnnouncer ||= announcements.createAnnouncer();
      record.pinAnnouncer.withdrawPending();
      record.pinAnnouncer.announce(record.element, record.tip.textContent || '',
        () => record.isPinned && record.element.isConnected && !!record.tip?.isConnected);
    }
  };
  // A hold shares the preview surface and dismissal owner, not a second manager.
  const show = record => {
    if (record.isPinned || record.isDisabled || record.element.closest('[inert],:disabled,[disabled],[aria-disabled="true"]')) return;
    showTip(record, record.tooltip);
    if (!record.tip) return;
    record.isPinned = true;
    record.tooltipPolicy?.onShow(record);
    announcePinned(record);
  };
  const refreshPolicyTip = record => {
    if (!record.tip) return;
    if (!record.tooltip?.trim()) { hideTip(record); return; }
    if (record.tip.textContent !== record.tooltip) {
      record.tip.textContent = record.tooltip;
      if (record.isPinned) announcePinned(record);
    }
    positionTip(record);
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
  const focusTip = record => {
    if ((record.pointerFocus && !record.focusTriggerAlways) || record.tooltipPolicy?.isHoverOnly(record)) return;
    if (record.isTruncation) return;
    if (record.tooltip) { showTip(record, record.tooltip); return; }
    if (record.isIconOnly || !record.element.matches(':focus-visible') || callerTooltip(record)) return;
    const label = record.element.querySelector('.astryx-button__label');
    if (label && !record.pointerFocus && isKeyboardModality) {
      const text = cutTextOnKeyboardFocus(label);
      if (text) showTip(record, text, true);
    }
  };
  listen(document, 'focusin', event => {
    const record = interestRecord(event.target);
    if (record && !record.isTruncation) { record.isFocused = true; focusTip(record); }
  }, { capture: true });
  listen(document, 'focusout', event => {
    const record = interestRecord(event.target);
    if (record && !record.isTruncation) {
      record.hadFocus = record.isFocused;
      record.isFocused = false; record.pointerFocus = false;
      if (!record.tip?.contains(event.relatedTarget)) hideTip(record);
      queueMicrotask(() => { if (record.element.isConnected) record.hadFocus = false; });
    }
  }, { capture: true });
  listen(document, 'pointerover', event => {
    const record = interestRecord(event.target);
    if (!record || record.isTruncation || record.element.contains(event.relatedTarget) || event.pointerType === 'touch' || !record.tooltip) return;
    cancelTimer(record, record.hideTimer);
    cancelTimer(record, record.showTimer);
    record.showTimer = later(record, () => showTip(record, record.tooltip), 200);
  }, { capture: true });
  listen(document, 'pointerout', event => {
    const record = interestRecord(event.target);
    if (!record || record.isTruncation || record.element.contains(event.relatedTarget)) return;
    cancelTimer(record, record.showTimer);
    if (record.tip && !record.isFocused && !record.isPinned) record.hideTimer = later(record, () => hideTip(record), 100);
  }, { capture: true });
  listen(document, 'pointerdown', event => {
    isKeyboardModality = false;
    const record = interestRecord(event.target);
    if (record && !record.isTruncation) {
      record.pointerFocus = true;
      // A standalone pin trigger's click decides; pressing a preview must not
      // flash it shut, and pressing a nested image must never open on focus.
      if (!record.isPinned && (!record.tooltipPolicy || record.tooltipPolicy.isHoverOnly(record))) hideTip(record);
    }
  }, { capture: true, passive: true });
  listen(document, 'keydown', event => {
    if (['Tab', 'Escape', 'ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(event.key)
        || !event.target.closest?.('input,textarea,[contenteditable]')) isKeyboardModality = true;
    for (const record of records.values()) {
      if (record.element.contains(event.target) || event.target.contains(record.element)) record.pointerFocus = false;
    }
  }, { capture: true });
  const removeTruncation = record => {
    hideTip(record);
    record.layerAbort?.abort(); record.boundTip = null;
    if (record.ownsTruncationDescription) {
      const ids = (record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(id => id && id !== record.retainedTip?.id);
      if (ids.length) record.element.setAttribute('aria-describedby', ids.join(' '));
      else record.element.removeAttribute('aria-describedby');
    }
    record.ownsTruncationDescription = false;
    record.pendingPresentation = null;
    record.truncationPresentation?.dispose(); record.truncationPresentation = null;
    record.retainedTip?.remove(); record.retainedTip = null; record.ownedTip = null;
  };
  listenForTruncationOutsideDismiss({ records, listen, hide: hideTip });
  for (const type of ['pointerup', 'pointercancel']) listen(document, type, event => {
    for (const record of records.values()) {
      if (record.isTruncation && !record.element.contains(event.target)) forgetPending(record);
    }
  }, {capture: true, passive: true});
  const truncationInterest = createTruncationInterest({ records, listen,
    preload(record) { preload(record).catch(() => {}); },
    mount(record, text) {
      record.truncationText = text;
      if (callerTooltip(record)) return false;
      for (let parent = record.element.parentElement; parent; parent = parent.parentElement) {
        if (records.get(parent)?.tooltip || callerTooltip({ element: parent })) return false;
      }
      showTip(record, text, true, true);
      return !!record.retainedTip || !!record.pendingPresentation;
    },
    hover(record, readText) {
      cancelTimer(record, record.hideTimer); cancelTimer(record, record.showTimer);
      whenReady(record, () => { record.showTimer = later(record, () => showTip(record, readText(), true), 200); }, 'interest');
    },
    leave(record, next) {
      forgetPending(record);
      cancelTimer(record, record.showTimer);
      if (!record.isTapOpen && !record.retainedTip?.contains(next)) record.hideTimer = later(record, () => hideTip(record), 100);
    },
    press(record, isTouch) {
      forgetPending(record);
      cancelTimer(record, record.showTimer); cancelTimer(record, record.hideTimer);
      if (!isTouch) hideTip(record);
    },
    focus(record, target, text) {
      record.isFocusCounted = isFocusOnTrigger(target, record.element);
      if (!record.isFocusCounted) return;
      cancelTimer(record, record.showTimer); cancelTimer(record, record.hideTimer);
      showTip(record, text, true);
    },
    blur(record, target) {
      forgetPending(record);
      if (!record.isFocusCounted && !isFocusOnTrigger(target, record.element)) return;
      record.isFocusCounted = false;
      record.hideTimer = later(record, () => hideTip(record), 100);
    },
    answerTap(record, target, text) {
      if (!modules?.loaded) { whenReady(record, () => truncationTap(record, target, text), 'interest'); return; }
      truncationTap(record, target, text);
    },
    remove: removeTruncation,
  });
  function truncationTap(record, target, text) {
      const field = target.closest?.('input,textarea,select,[contenteditable]:not([contenteditable="false"]),[role="textbox"],[role="searchbox"],[role="spinbutton"],[role="combobox"],[role="listbox"],[role="slider"]');
      const disabled = target.closest?.(':disabled,[aria-disabled="true"]');
      if (field && field !== record.element && !disabled) return;
      if ((!disabled && isActionTrigger(record.element)) || record.isTapOpen || record.tip) { hideTip(record); return; }
      showTip(record, text, true);
      record.isTapOpen = !!record.tip;
  }
  // Components own trigger semantics; Tooltip only supplies shared presentation
  // and show/hide capabilities. Cache one component policy per manager lifetime.
  const policies = new Map();
  return {
    hideTip, focusTip, syncDescription, showTip, show, refreshPolicyTip,
    attach(record) {
      truncationInterest.attach(record);
      if (!record.tooltipPolicyFactory) return;
      let policy = policies.get(record.tooltipPolicyKey);
      if (!policy) {
        policy = record.tooltipPolicyFactory({ records, listen, show, hide: hideTip });
        policies.set(record.tooltipPolicyKey, policy);
      }
      record.tooltipPolicy = policy;
      policy.attach(record);
    },
    update(record) { record.tooltipPolicy?.update(record); },
    command(record, command) { record.tooltipPolicy?.command(record, command); },
    dispose(record) {
      hideTip(record);
      record.tooltipPolicy?.dispose(record);
      truncationInterest.dispose(record);
      if (record.isTruncation) removeTruncation(record);
      record.layerAbort?.abort();
      record.ownedTip?.remove();
      record.pinAnnouncer?.dispose(); record.pinAnnouncer = null;
      if (!record.tooltipPolicy && !record.isTruncation) {
        const ids = (record.element.getAttribute('aria-describedby') || '').split(/\s+/).filter(id => id && id !== `${record.id}-tooltip`);
        if (ids.length) record.element.setAttribute('aria-describedby', ids.join(' '));
        else record.element.removeAttribute('aria-describedby');
      }
    },
    dismissOutside(target) {
      if (visibleTip && !visibleTip.isTapOpen && !visibleTip.element.contains(target) && !visibleTip.tip?.contains(target)) hideTip(visibleTip);
    },
    escape(event) {
      if (event.key === 'Escape') for (const record of records.values()) forgetPending(record);
      if (event.key === 'Escape' && !event.isComposing && !event.defaultPrevented && visibleTip) { hideTip(visibleTip); event.preventDefault(); event.stopPropagation(); }
    },
    reposition() { if (visibleTip) positionTip(visibleTip); },
    hide() { if (visibleTip) hideTip(visibleTip); },
  };
};
