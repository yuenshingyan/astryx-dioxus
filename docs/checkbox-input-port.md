# CheckboxInput checkpoint

CheckboxInput composes a controlled native checkbox with CheckboxIndicator, FieldLabel, and optional detached FieldStatus. The browser owns keyboard activation and form participation. The port synchronizes the native mixed state and restores the caller's value when a change is declined. An unavailable checkbox remains focusable, rejects activation, and does not participate in its enclosing form.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CheckboxInput/CheckboxInput.tsx` (`749516c8c3ea7d2b2c705013c80b8607aa85c10b`), its spec (`75b870b4d1c329a2f2306deb29f3a583d4a9b1b1`), and four adapted catalog arrangements. Async change actions, busy-state announcements, disabled-message help, broader native attribute forwarding, and touch press geometry remain open.

All five standard Rust checks passed, along with the focused SSR test and three Chrome suites. Chrome verified mixed state, Space activation, controlled restore, accessible checkbox naming, and disabled form exclusion. The loading follow-up adds caller-controlled `is_loading`, which keeps the checkbox focusable, blocks activation, exposes `aria-busy`, and replaces the mark with a decorative Spinner. The local demo simulates loading without a request. The CSS bundle check, all five Rust checks, and the CheckboxInput, CheckboxList, Switch, and showcase-navigation Chrome suites passed. Chrome verified that clicks and Space do not change the value while busy, and Space works again afterward. The Spinner motion suite was not rerun. Published visual parity and manual assistive technology were not verified. The checklist remains open.

## Explicit required follow-up

Pinned FR7: `is_required` now exposes `aria-required="true"` on the native checkbox alongside the existing `required` attribute. An unmarked checkbox has neither attribute. `aria-invalid` remains status-driven (`false` for a required checkbox without an error), without bypassing native constraint validation or changing form submission or caller-owned state.

The focused `explicit_required_preserves_native_validation_and_reports_status` SSR test passed, covering unmarked, required without error, and required with error markup. Formatting and style bundle verification passed. Browser submission and accessibility-tree behavior were not rechecked for this follow-up.

## Enabled-row hover follow-up

The pinned TSX applies `indicatorScope` to the inner row whenever the checkbox is not disabled, including while loading. This port uses the existing `data-astryx-indicator-scope` selector on that row; no styles, activation guards, or indicator APIs changed. The supplied TSX and spec match the pinned blob IDs above.

The focused SSR suite passed (3 tests). The new regression covers all four disabled/loading combinations: scope placement, disabled-state form exclusion attributes, and busy/Spinner markup. The browser regression moves the pointer over the label rather than the native input, checks that enabled and loading rows tint but disabled rows do not, and exercises click/Space blocking and recovery from loading. Run `node tests/checkbox_input.browser.mjs` against the showcase and Chrome CDP to verify that behavior. Only its syntax was checked here; the integrator owns the browser run. No visual comparison or spoken accessibility parity was verified. Host press treatment and replacement-root focus remain outside this follow-up.
