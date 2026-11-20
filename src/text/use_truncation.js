// Adapted from Meta Astryx Text/useTruncation.ts at
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.
const truncationKey = Symbol.for('astryx.dioxus.truncation');
const createTruncation = (notify) => {
  let element = null;
  let keptFor = null;
  let maxLines = 0;
  let isExpanded = false;
  let answer = { isTruncated: false, fullText: '' };
  let unobserveResize = null;
  let unobserveContent = null;
  let unobserveRemoval = null;
  const publish = () => {
    if (element) {
      element[truncationKey] = answer;
      element.dispatchEvent(new Event('astryxtruncationchange'));
    }
    notify(answer.isTruncated, answer.fullText);
  };
  const stop = () => {
    unobserveResize?.(); unobserveResize = null;
    unobserveContent?.(); unobserveContent = null;
    unobserveRemoval?.(); unobserveRemoval = null;
  };
  const check = () => {
    if (!element?.isConnected || maxLines === 0 || isExpanded) return;
    const fullText = element.textContent || '';
    let isTruncated;
    if (maxLines === 1) {
      isTruncated = element.scrollWidth > element.offsetWidth;
    } else {
      let contentHeight = element.scrollHeight;
      try {
        const range = element.ownerDocument.createRange();
        range.selectNodeContents(element);
        contentHeight = range.getBoundingClientRect().height;
        range.detach();
      } catch { /* Non-layout renderers retain the source scrollHeight fallback. */ }
      isTruncated = contentHeight > element.offsetHeight;
    }
    if (answer.isTruncated !== isTruncated || answer.fullText !== fullText) {
      answer = { isTruncated, fullText };
      publish();
    }
  };
  const attach = (next, lines, expanded) => {
    if (next === element && lines === maxLines && expanded === isExpanded) return;
    stop();
    const previous = element;
    if (expanded && !isExpanded) keptFor = previous;
    if (!expanded && previous !== next) answer = { isTruncated: false, fullText: '' };
    if (previous && previous !== next) delete previous[truncationKey];
    element = next;
    maxLines = lines;
    isExpanded = expanded;
    if (isExpanded) {
      if (element) publish();
      else keptFor = previous || keptFor;
    } else {
      if (keptFor && keptFor !== element) answer = { isTruncated: false, fullText: '' };
      keptFor = null;
      if (!element || maxLines === 0) answer = { isTruncated: false, fullText: '' };
      publish();
      if (element && maxLines > 0) {
        if (element.ownerDocument.defaultView?.ResizeObserver) {
          unobserveResize = observeResize(element, check, { afterLayout: true });
        } else check();
        unobserveContent = observeContent(element, check);
      }
    }
    if (element && (maxLines > 0 || isExpanded)) {
      unobserveRemoval = observeContent(element.ownerDocument, () => {
        if (!element?.isConnected) attach(null, maxLines, isExpanded);
      });
    }
  };
  return {
    attach,
    dispose() {
      stop();
      if (element) delete element[truncationKey];
      element = null;
    },
  };
};
