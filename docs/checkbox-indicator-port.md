# CheckboxIndicator port

`CheckboxIndicator` paints a persistent decorative box in unchecked, checked, or indeterminate state. The root owns its reflected size and state and enforces `aria-hidden`; the host retains control semantics, focus, and activation. The box, check, and dash keep independent theme targets. A supplied `replacement: Option<Element>` occupies the box instead of either mark, including when deliberately empty.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Indicator/CheckboxIndicator.tsx` (Git blob `ca8594e71c04c6a3a80c2e6a06e1b0ff9fe8546c`). Host-scoped hover uses the existing `data-astryx-indicator-scope` marker. The pinned tree has no standalone CheckboxIndicator CLI templates; the showcase provides state and styling examples. Public checkbox controls and the theme replacement registry are separate migrations.

All five standard Rust checks passed. Chrome verified box and mark sizes, replacement content, scoped hover, reduced motion, and distinct check and dash paint in forced colors. RadioIndicator, CheckIndicator, and showcase navigation checks also passed. Published screenshot parity, other browsers, and manual assistive technology were not tested. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8122 node tests/checkbox_indicator.browser.mjs
```
