# Switch checkpoint

`src/switch/` renders a controlled native checkbox with switch semantics, a FieldLabel, and optional description and detached status. Disabled switches stay focusable, block activation, and are excluded from form submission. The track supports two sizes, and logical label placement and thumb movement mirror in RTL.

Sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Switch/Switch.tsx` (`8e18e1cc587bfd8e0fbecbd83b676bec7d5185f0`) and its spec (`c3adfb5900f3758984b2c8117c24a97df4364b6d`). The showcase adapts the basic, description, and disabled arrangements. Drag and touch press behavior, async state, disabled-reason help, remaining catalog arrangements, and published visual parity remain open.

All five Rust checks and four Chrome suites passed. SSR verified label and description associations and disabled form markup. Chrome verified Space activation, restoration when the caller declines a change, disabled focus and form behavior, accessible name and checked state, and RTL thumb movement. Spoken output was not verified; the checklist remains open.
