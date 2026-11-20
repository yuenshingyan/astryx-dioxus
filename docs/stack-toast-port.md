# Private Stack demo Toast slice

This is **published Stack demo parity, not a public Toast migration**. No Toast API
is exported from `src/lib.rs`. Existing public APIs, dependencies, static pattern
frames, and older visual fixtures are unchanged.

## Authority and ownership

Meta Astryx pin: `476a3153417dbf75c565f7df05b0784e94ddd59e`.
`tests/fixtures/stack-toast/source.json` preserves twelve original files and their
Git blob hashes: Toast's spec, Toast/ToastViewport/useToast/useToastGesture,
MediaTheme/onMediaTokens, shared tokens, Neutral theme/palette, and both Stack
button templates. These were verified against the existing full reference tree,
not inferred from the deployed version. `tests/stack_toast_source.py --check`
rechecks fixture digests and, when available, that tree's pin and blob entries.

The pinned Stack templates have **no action callbacks**. Published code captured
2026-11-20T10:05:53.671Z at <https://astryx.atmeta.com/components/Stack> displays
`@astryxdesign/core v0.7.16` and adds `useToast` callbacks. The deployment commit is
unknown; displayed version does not establish equivalence to the pin.
`tests/fixtures/stack-toast/published.json` contains the actual open Toast DOM,
computed styles, displayed code, capture identity and interaction observations;
three PNG clips have SHA256 digests. Catalog access is opt-in capture only.

`examples/showcase/stack.rs` still owns the exact Cancel/Save/View messages and
button callbacks. Its old probe output is hidden and not live, preventing a second
announcement. Private `src/toast/pattern.rs`, `pattern_viewport.js` and
`pattern_toast.js` retain Toast's card/viewport ownership. Theme owns
`src/theme/pattern_media.js` and the private media overrides; shared
`hooks/use_announce` still owns the polite writer. Existing Button/Icon render the
close control. Shared CSS stays in `assets/astryx.css`; the missing on-light tokens
come from verified core defaults (`#000`) and Neutral's light palette step 5
(`#111111`), not a guessed Toast palette.

## Implemented slice

- Default info/body-only card: 400px maximum width, token padding/radius/shadow,
  first-line trailing 28px ghost dismiss control, measured media text side.
- Default bottomEnd top-layer viewport, safe-area gutters, newest nearest edge,
  token 8px row gaps, five visible; hidden older entries return when slots open.
- Independent 5000ms timers; hover, focus, gesture and window holds, remaining-time
  resume with a 1000ms floor; already-blurred shows start held.
- Entry/exit grid collapse and opacity/block translation at theme timing; clipped
  while transitioning, shadow released when settled; pinned 0.01ms reduced motion.
- One shared polite announcement owner, per-entry identity, no announcement from
  the card or hidden probe, and no unrelated-render repetition. Hidden entries
  announce when first shown; resurfacing does not re-announce an already shown ID.
- F6 enters newest Toast; dismiss transfers focus to another Toast or the caller.
  Touch/pen swipe constants and bottom-directed intent follow the pinned shipped
  rule; this does not claim to repair the spec's documented gesture-runtime gaps.
- Row disposal clears timers/listeners/theme observers; viewport disposal removes
  its F6/focus listeners, observer, pending announcement shares and top-layer box.

## Actual comparisons and tests

All browser sessions used `browser_lock.py`, only owned CDP targets, and local
port **8098**; the blur test opens/closes a second owned blank target. No shared
server or existing user target was changed. Local comparisons run offline after
loading bundled assets. Chrome 157.0.8103.80, 1200×1600, DPR1, 16px root,
Neutral dark/LTR:

- All three actual Rust Toasts match the published **400×52px**, x=784/y=1532 box
  (16px bottom/end gutters). Card, inner, body, trailing row, Button, Icon and SVG
  geometry match within 1/64px. Selected background/ink/radius/shadow and
  size/weight/line-height computed values match exactly. No frozen replay is used.
- Published F6/Enter dismissal returns to the caller; hover holds beyond 5.3s;
  unheld auto-hide including exit was observed at **5319ms** (polling, not a timer
  precision guarantee); six shows expose five and oldest resurfaces. Local tests
  exercise those same behaviors plus independent queue identities/gaps.
- Local tests additionally exercise separate-turn identical-message polite DOM
  writes, unrelated-render silence, overlapping real hover/focus holds, late-focus
  1000ms floor, real owned-tab blur/show-while-blurred/resume, CDP touch short
  snap-back/52px dismissal, inherited mode changes, 320px RTL/reduced motion,
  and timed unmount followed by 5.3s without an orphan Toast or JS exception.
  DOM live-region writes are **not screen-reader speech tests**.
- Two SSR tests cover empty landmark omission and caller body/accessible labels/
  existing Button variant/size; the existing Stack pattern SSR test also passes.

Actual screenshots/metrics and command logs are published locally under
`/tmp/astryx-parity/toast-actual` and `/tmp/astryx-parity/toast-*.log`. The separate
stdlib PNG comparison found nonzero different-pixel fractions: Cancel **7.505%**,
Save **7.120%**, View **6.250%**; mean absolute channel errors **1.098/1.067/0.892**,
max **85**. Clips include their respective surrounding surfaces and text raster;
this is not a zero-raster-difference or full-page visual certificate.

Reproduce (start the owned preview separately):

```sh
CARGO_INCREMENTAL=0 dx serve --example stack --web --addr 127.0.0.1 --port 8098 --open false --interactive false --watch false --locked
python3 tests/stack_toast_source.py --check
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_toast.browser.mjs
python3 tests/stack_toast_pixels.py
# Explicit network opt-in, replaces only Toast-specific capture fixtures:
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_toast.capture.mjs
```

## Local check results

Passed with `CARGO_INCREMENTAL=0`: `cargo fmt --check`, `cargo check --locked --lib
--no-default-features`, `cargo test --locked` (229 tests across 32 reported test
results, including doctests), `cargo clippy --locked --all-targets -- -D warnings`,
and `cargo check --locked --example showcase --example stack --target
wasm32-unknown-unknown`. Source/image integrity, the serialized browser suite,
PNG dimension/integrity recording and syntax checks in the actual async-eval
wrapper also passed. The initial new nested-if Clippy finding was fixed, not
suppressed. Plain Node CommonJS `--check` is inapplicable to these async eval
bodies (top-level await/return); the correct wrapper syntax check and real browser
execution were used instead. Dioxus's existing document::Style prop-change
warnings remain visible during example rerenders; no browser JS exception was
recorded. Static Stack browser suites that rewrite another agent's visual evidence
and unrelated existing browser/contrast suites were not run. CI must still run the
full existing suite.

## Explicit limits

Not implemented or certified: public Toast/useToast/provider APIs, error/custom
content/action/collision options, alternative placements/insets, cross-root host
queues, modal rehosting, localization runtime, arbitrary custom-theme/media color
syntax, general shared gesture runtime, or full public MediaTheme. No new
application operations were invented. Pen, interactive-descendant swipe exclusion,
second-touch cancellation, sideways/opposite/fast flick cases, safe-area/coarse
hardware, forced colors, screen readers, Safari/Firefox and arbitrary themes were
**not browser-tested**. Published interactions beyond the listed capture cases
were not exercised. Full upstream React tests were not run. Documented upstream
spec/source discrepancies are not silently called fixed.
