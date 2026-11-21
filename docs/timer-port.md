# Timer checkpoint

`src/timer/` ports elapsed and clock formats using Text for typography and native `<time datetime="…">` for the duration. Server rendering starts at zero. In the browser, the timer bridge updates the existing text node and duration attribute at display boundaries without scheduling Dioxus renders. It reads wall time on every tick, switches compact elapsed output to minute precision after an hour, and cancels the pending timeout on unmount. It does not create a live region.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Timer.tsx` (Git blob `141a7d65f04b0c814a70ada5005a04f3efa31786`), `Timer.spec.md` (`a52d290b7b90acd5d2819647505eb7da8fcda624`), and all four catalog examples.

All five standard Rust checks passed. The deterministic scheduler test covered display boundaries, delayed ticks, future starts, and cancellation. Chrome verified native duration updates, stable DOM node identity across format changes, and unmount; Thumbnail and showcase navigation regressions passed. Published screenshot parity, other browsers, and manual assistive technology were not verified. The migration checklist remains open.

```sh
node tests/timer_scheduler.mjs
SHOWCASE_URL=http://127.0.0.1:8132 node tests/timer.browser.mjs
```
