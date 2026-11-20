# RadioIndicator port

`RadioIndicator` is a decorative circle for a caller-owned selection, not a form control. Both states retain the circle. A selected circle adds the inner dot unless the caller supplies replacement content. The `replacement: Option<Element>` prop distinguishes absence from deliberate empty content. The root owns `aria-hidden`, reflected size and state, and forwards native attributes, styling, and callbacks. Its circle and dot retain independent theme classes.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Indicator/RadioIndicator.tsx` (Git blob `eb6bc709c7402694aaa1baec5eab80bc9c087d2b`). A host can opt into hover paint with `data-astryx-indicator-scope`; the indicator does not own a focus ring or interaction state. No public RadioList or theme replacement system is claimed. The pinned tree has no standalone RadioIndicator CLI template, so the showcase uses contract examples.

All five standard Rust checks passed. Chrome verified state geometry, replacement content, scoped hover, reduced motion and the selected dot in forced colors. CheckIndicator and showcase navigation regression checks also passed. Published screenshots, other browsers, and manual assistive technology were not tested; the migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8121 node tests/radio_indicator.browser.mjs
```
