# GridSpan port

GridSpan is the static participation modifier for Grid. It renders one native `div` with no added role, name, or tab stop. Its inner grid has one `minmax(0, 1fr)` track, zero minimum width, and full height. `GridSpanColumns::Count` spans a numeric count, `Full` covers only the explicit columns (`1 / -1`), and `rows` controls the row span. Positive spans override ordinary inline placement; unset or zero counts leave caller placement unchanged. Caller children retain DOM order and semantics. Native attributes and `onmounted` reach the wrapper.

The port follows [Meta Astryx `476a315`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), including `Grid/GridSpan.tsx` (blob `7c109c52923e91c4e306fe07faf4a260703fc2db`) and `GridSpan.spec.md` (blob `2e2be77a4e57c5e60372f26b4f968440abf0c08d`). The showcase includes the original Showcase and Columns patterns. It also replaces the temporary span wrappers in Grid's Dashboard and WithGridSpan examples.

The five standard Rust checks passed. SSR tests cover placement precedence, root attributes and DOM order. Chrome verified both patterns, numeric and full spans, row spans, LTR/RTL, and the nonsemantic wrapper. The existing Grid and showcase navigation checks also passed. Independent upstream screenshot comparison, other browsers, and manual assistive-technology checks were not run. GridSpan is not marked complete in the migration checklist.

```sh
SHOWCASE_URL=http://127.0.0.1:8113 node tests/grid_span.browser.mjs
```
