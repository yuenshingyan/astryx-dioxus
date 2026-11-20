# LayoutContent port

`LayoutContent` renders one native `div` that owns the scrollport and holds caller children directly. It scrolls by default and clips overflow when scrolling is disabled. An overflowing scrollport without a caller role gains `role="group"` and a name from the caller label or `scroll_label` (default: “Content”). It adds no tab stop, and a caller role takes precedence. StackItem and LayoutContent share the internal overflow measurement hook.

Padding defaults to the Layout edge inset where a slot is absent and the corresponding inner inset where a slot is present. An explicit spacing step overrides those base insets; zero also disables alignment gutters. The scrollport keeps its width while inline padding aligns its children to the parent Layout's width constraint. The base inset remains published separately from the alignment gutter. Block bleed is limited to eligible direct edge children. The slot context resets at a layer boundary.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Layout/LayoutContent.tsx` (Git blob `414d95797e783e4573daf8af17976d4c1a5bf853`). Basic and Showcase retain the original content and use the public LayoutHeader and LayoutFooter where applicable. Parent Layout and LayoutPanel are demo framing; the demos use named regions rather than nesting `main` landmarks in the showcase. The parent Layout port will supply slot presence, width constraints, and adjacent-edge collapse in production composition.

All five standard Rust checks passed. Chrome verified both arrangements, overflow-based naming and removal, scrolling, inherited insets, width gutters, and full bleed. LayoutHeader, LayoutFooter, StackItem, and showcase navigation checks also passed. Published screenshot parity, other browsers, and manual assistive technology were not tested; the migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8126 node tests/layout_content.browser.mjs
```
