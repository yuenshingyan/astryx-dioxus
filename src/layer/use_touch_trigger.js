const listenForTruncationOutsideDismiss = ({ records, listen, hide }) => {
  const end = event => {
    for (const record of records.values()) {
      if (record.isTruncation && record.isTapOpen && !record.retainedTip?.contains(event.target)
          && (event.type === 'pointercancel' || !record.element.contains(event.target))) hide(record);
    }
  };
  listen(document, 'pointerup', end, { capture: true });
  listen(document, 'pointercancel', end, { capture: true });
  listen(document, 'scroll', () => {
    for (const record of records.values()) if (record.isTruncation && record.isTapOpen) hide(record);
  }, { capture: true, passive: true });
};
// Private auto-mode judgement from Layer/useTouchTrigger.ts; not a public Layer port.
const isActionTrigger = element => {
  if (element.matches(':disabled,[aria-disabled="true"]')) return false;
  const role = element.getAttribute('role');
  if (role) return ['button','checkbox','combobox','link','menuitem','menuitemcheckbox','menuitemradio','option','radio','searchbox','slider','spinbutton','switch','tab','textbox'].includes(role);
  if (element.tagName === 'LABEL') return !element.control || !element.control.matches(':disabled,[aria-disabled="true"]');
  return element.matches('button,input,select,summary,textarea,a[href],area[href]') || element.isContentEditable;
};
