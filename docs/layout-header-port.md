# LayoutHeader port

`LayoutHeader` renders a non-shrinking outer band and a centered inner content box. Height, caller styling, native attributes, and mounted callbacks target the outer band. No landmark is added by default. The optional divider is a logical block-end border; an explicit choice overrides the private Layout default, which stops at `LayerContentBoundary`.

The inner wrapper consumes the Layout width and inset variables, publishes the padding it applies, and only grants block bleed to eligible direct edge children. A per-edge block-end override wins over the uniform padding, including when set to zero.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Layout/LayoutHeader.tsx` (Git blob `c1f8e356e5bf1b5c0eed85cb20c499a18e484a35`). The two original header arrangements and action messages are represented in the showcase. Supporting Layout, LayoutPanel, LayoutContent, and LayoutFooter are demo framing only; the actions use inline status messages rather than Toasts. Layout-owned spacing collapse is not part of this port.

All five standard Rust checks passed. Chrome verified both header arrangements, the Export action message, inherited insets, the content-width cap, block bleed, and logical divider placement. Card and showcase navigation checks also passed. Published screenshot parity, other browsers, and manual assistive technology were not tested. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8124 node tests/layout_header.browser.mjs
```
