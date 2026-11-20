# VisuallyHidden port

`VisuallyHidden` clips content out of sight without adding `aria-hidden` or changing its semantics. It defaults to a `span`; a custom renderer must forward the supplied attributes and children to one root. Native IDs, ARIA, data attributes and callbacks reach that root. The released component's fixed clip protection is preserved: caller class and inline style attributes do not replace it. Callers can still remove content from the accessibility tree explicitly, for example with `hidden`.

Thirteen upstream source, spec, story, test, and template files were verified against Meta Astryx revision `476a3153417dbf75c565f7df05b0784e94ddd59e`. `VisuallyHidden.tsx` is Git blob `dcbe3f5019e5bf94adfd0df35f8d7e2f81c8c6d1`. The showcase includes all four original patterns: Structural Heading, Live Region, Supplementary Context, and Showcase. ProgressBar continues to use the same clipping styles.

SSR tests cover the default root, native attributes, fixed style, and custom heading. Chrome tests verify clipping, heading and button names, supplementary text in the accessibility tree, and updates to the persistent live region. The tests do not verify spoken announcements. Published screenshot comparison, other browsers, and manual assistive technology were not tested; the migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8119 node tests/visually_hidden.browser.mjs
```
