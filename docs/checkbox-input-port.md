# CheckboxInput checkpoint

CheckboxInput composes a controlled native checkbox with CheckboxIndicator, FieldLabel, and optional detached FieldStatus. The browser owns keyboard activation and form participation. The port synchronizes the native mixed state and restores the caller's value when a change is declined. An unavailable checkbox remains focusable, rejects activation, and does not participate in its enclosing form.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CheckboxInput/CheckboxInput.tsx` (`749516c8c3ea7d2b2c705013c80b8607aa85c10b`), its spec (`75b870b4d1c329a2f2306deb29f3a583d4a9b1b1`), and four adapted catalog arrangements. Async change actions, loading, disabled-message help, broader native attribute forwarding, and touch press geometry remain open.

All five standard Rust checks passed, along with the focused SSR test and three Chrome suites. Chrome verified mixed state, Space activation, controlled restore, accessible checkbox naming, and disabled form exclusion. Published visual parity and manual assistive technology were not verified. The checklist remains open.
