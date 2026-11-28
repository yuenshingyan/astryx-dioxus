# LayoutFooter port

`LayoutFooter` renders a non-shrinking bottom band with a centered inner box. The outer band receives sizing, caller styling, native props, and mounted callbacks. It adds no landmark by default. The optional logical block-start divider uses the shared Layout default unless the caller overrides it; the default stops at `LayerContentBoundary`.

The inner box applies and publishes the Layout insets. It uses the inner inset at block start and the outer inset at block end. An explicit padding step overrides all four edges, including zero. Only eligible direct edge children receive block bleed.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Layout/LayoutFooter.tsx` (Git blob `34c91e73fbea3ee5de8521d77346ec4d5f73f066`) and `LayoutFooter.spec.md` (`64c72c6ffc2358eb2c1a8b14897b443ad4dd7033`). Both footer arrangements now use the existing Layout footer slot and a bounded LayoutContent scrollport. The showcase with a sidebar also uses the header and logical start slots. The frame height is demo-only; actions still show inline status messages rather than Toasts. Middle-row spacing collapse remains Layout-owned.

At the original checkpoint, the five standard Rust checks and Chrome probes for the demo-framed arrangements passed. That browser result does not cover the new slot composition. The focused SSR regression checks both updated arrangements' markup, inherited divider reflection, and absence of a footer landmark role. The updated browser probe is not yet run: it tests scroll pinning and action messages separately from computed inset publication, bleed, width cap, and the logical divider. Neither SSR nor those computed-style assertions establish browser AX behavior, published raster parity, or screen-reader speech. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8125 node tests/layout_footer.browser.mjs
```
