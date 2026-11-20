# LayoutFooter port

`LayoutFooter` renders a non-shrinking bottom band with a centered inner box. The outer band receives sizing, caller styling, native props, and mounted callbacks. It adds no landmark by default. The optional logical block-start divider uses the shared Layout default unless the caller overrides it; the default stops at `LayerContentBoundary`.

The inner box applies and publishes the Layout insets. It uses the inner inset at block start and the outer inset at block end. An explicit padding step overrides all four edges, including zero. Only eligible direct edge children receive block bleed.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Layout/LayoutFooter.tsx` (Git blob `34c91e73fbea3ee5de8521d77346ec4d5f73f066`). Both original footer arrangements and action messages are included. The surrounding Layout, panel, and content use demo-only framing; actions show inline status messages rather than Toasts. Parent Layout spacing collapse is outside this port.

All five standard Rust checks passed. Chrome verified the arrangements and actions, inherited and zero padding, width cap, block bleed, and logical divider. LayoutHeader and navigation regression checks passed. Published screenshot parity, other browsers, and manual assistive technology were not tested; the migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8125 node tests/layout_footer.browser.mjs
```
