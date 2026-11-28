# LayoutHeader port

`LayoutHeader` renders a non-shrinking outer band and a centered inner content box. Height, caller styling, native attributes, and mounted callbacks target the outer band. No landmark is added by default. A forwarded `aria-label` takes precedence over `label`, with only one attribute emitted. The optional divider is a logical block-end border; an explicit choice overrides the private Layout default, which stops at `LayerContentBoundary`. The caller cannot override the divider's reflected `data-divider` attribute.

The inner wrapper consumes the Layout width and inset variables, publishes the padding it applies, and only grants block bleed to eligible direct edge children. A per-edge block-end override wins over the uniform padding, including when set to zero.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Layout/LayoutHeader.tsx` (Git blob `c1f8e356e5bf1b5c0eed85cb20c499a18e484a35`) and `LayoutHeader.spec.md` (`5716d7a8609b4f4b9fe1e1efed48f974470f029c`). The two original header arrangements and action messages are represented in the showcase. Supporting Layout, LayoutPanel, LayoutContent, and LayoutFooter are demo framing only; the actions use inline status messages rather than Toasts. Layout-owned spacing collapse is not part of this port.

At the earlier checkpoint, the standard Rust checks and Card and showcase navigation checks passed. The Chrome suite exercised both arrangements and the Export message in the DOM, and used computed CSS to check inherited insets, the content-width cap, a block-bleed value, and logical border placement. It did not check AX or raster parity.

`tests/layout_header.rs` covers native root markup, name and divider attribute precedence, distinct padding inputs, and the virtual DOM listener targets for mount and click. These are SSR and virtual DOM checks, not browser DOM/AX, computed geometry, raster, or screen-reader speech checks. Remaining geometry evidence includes applied versus published padding in real nested containers, width centering, and actual child docking at the divider. No published screenshot comparison or manual screen-reader run has been made. Neither named template is signed off by arrangement checks alone; the migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8124 node tests/layout_header.browser.mjs
```
