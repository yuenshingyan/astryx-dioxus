// Required slice of Meta Astryx motion/uikit/timing.ts at
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.
// CAMediaTimingFunction easeInEaseOut; measured UIView.animate, iOS 26.2,
// Oct 23 2026. Curves are not scaled by theme tempo.
const timing = Object.freeze({
  curves: Object.freeze({ move: Object.freeze([0.42, 0, 0.58, 1]) }),
});
