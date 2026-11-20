// Required native timing reader from Meta Astryx motion/themeTiming.ts at
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.
// Resolve calc()/var() and arbitrary browser-supported easing without layout.
const readThemeTimingTokens = (element, durationToken, easingToken, fallback = null) => {
  const view = element.ownerDocument.defaultView;
  if (!view) return { duration: 0, easing: 'linear' };
  const style = view.getComputedStyle(element);
  const durationText = style.getPropertyValue(durationToken).trim() || fallback?.missingDuration || fallback?.duration || '410ms';
  const easingText = style.getPropertyValue(easingToken).trim() || fallback?.easing || 'cubic-bezier(0.24, 1, 0.4, 1)';
  const probe = element.ownerDocument.createElement('span');
  probe.style.setProperty('display', 'none', 'important');
  if (fallback) {
    probe.style.setProperty('transition-duration', fallback.duration, 'important');
    probe.style.setProperty('transition-timing-function', fallback.easing, 'important');
  }
  probe.style.setProperty('transition-duration', durationText, 'important');
  probe.style.setProperty('transition-timing-function', easingText, 'important');
  // Remain inside the motion's token scope. No box or external text is read.
  element.append(probe);
  try {
    const resolved = view.getComputedStyle(probe);
    const match = /^([\d.eE+-]+)(ms|s)$/.exec(resolved.transitionDuration);
    const duration = match ? Number(match[1]) * (match[2] === 's' ? 1000 : 1) : NaN;
    return {
      duration: Number.isFinite(duration) && duration >= 0 ? duration : fallback?.milliseconds ?? 0,
      easing: resolved.transitionTimingFunction,
    };
  } finally {
    probe.remove();
  }
};
