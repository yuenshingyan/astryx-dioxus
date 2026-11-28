# Timer checkpoint

`src/timer/` ports elapsed and clock formats using Text for typography and native `<time datetime="…">` for the duration. Server rendering starts at zero. In the browser, the timer bridge updates the existing text node and duration attribute at display boundaries without scheduling Dioxus renders. It reads wall time on every tick, switches compact elapsed output to minute precision after an hour, and cancels the pending timeout on unmount. It does not create a live region.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Timer.tsx` (Git blob `141a7d65f04b0c814a70ada5005a04f3efa31786`), `Timer.spec.md` (`a52d290b7b90acd5d2819647505eb7da8fcda624`), and all four catalog examples.

All five standard Rust checks passed. The deterministic scheduler test covered display boundaries, delayed ticks, future starts, and cancellation. Chrome verified native duration updates, stable DOM node identity across format changes, and unmount; Thumbnail and showcase navigation regressions passed. Published screenshot parity, other browsers, and manual assistive technology were not verified. The migration checklist remains open.

## Timing and cleanup follow-up

Against the pinned source and spec (FR4, FR9, PR2), the Timer bridge now keeps its pending boundary when it receives the same effective origin and format twice. Previously, the mount callback and initial reactive effect could schedule the same configuration twice, canceling and recreating the timeout even though nothing changed. If the mount lookup misses, the bridge now falls through to Dioxus's evaluator channel close instead of returning before it.

The deterministic scheduler test checks one pending timeout, unchanged configuration without new scheduling or writes, origin and format changes, reuse of the mount origin, future-start timing, cancellation, and the missing-node close path. This is not live renderer evidence: the integrator still needs to run the browser regression for mounted prop updates and unmount. Published visual comparisons and manual assistive-technology checks remain open; no checklist items were promoted.

```sh
node tests/timer_scheduler.mjs
SHOWCASE_URL=http://127.0.0.1:8132 node tests/timer.browser.mjs
```
