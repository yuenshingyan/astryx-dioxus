// Adapted from Meta Astryx motion/uikit/progressBar.ts at
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.
const progressBar = Object.freeze({
  // Measured UIKit reference, retained but NEVER played by ProgressBarTouch.
  // iOS 26.2 simulator, UIProgressView, Oct 23 2026.
  fillTrackMs: 1000,
  curves: Object.freeze({ fill: Object.freeze([0, 0, 1, 1]) }),
  // Owner values, not UIKit measurements (upstream M-PB1). Busy loop: fixed.
  reducedMotionPulseMs: 2000,
  reducedMotionPulseFloor: 0.84,
  progressView: Object.freeze({
    heightPt: 4,
    minimumFillHeights: 2,
    emptyFillAlpha: 0,
  }),
});

// The required slice of the internal role table. No public Motion API implied.
const progressBarMotionRoles = Object.freeze({
  'progress.fillTrackMs': progressBar.fillTrackMs,
  'progress.curves.fill': progressBar.curves.fill,
  'progress.reducedMotionPulseMs': progressBar.reducedMotionPulseMs,
});
