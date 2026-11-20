// Adapted from utils/triggerFocus.ts. Evaluated when focus arrives, not on attach.
const isFocusableTrigger = element => element.hasAttribute('tabindex') ? element.tabIndex >= 0
  : ['A', 'BUTTON', 'INPUT', 'SELECT', 'TEXTAREA'].includes(element.tagName) ? !element.disabled : element.isContentEditable;
const isFocusOnTrigger = (target, trigger) => target === trigger || isFocusableTrigger(trigger);
