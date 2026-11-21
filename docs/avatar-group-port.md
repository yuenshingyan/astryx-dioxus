# AvatarGroup checkpoint

`src/avatar_group/component.rs` renders a named group. The caller selects which avatars and overflow indicator to render. The group publishes size, shape, and quarter-size overlap through a layer-scoped context. Static members add no default tab stops inside the group. Interactive members share one tab stop; Left/Right wrap in reading order, and Home/End jump to the first or last member. The keyboard hint supplements any caller description. The browser controller cleans up its observer and listeners on unmount.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `AvatarGroup/AvatarGroup.tsx` (Git blob `c3d76c1f4da82e26a6ee7f987c379e1fe44727b8`), the component spec, and both showcase templates. The overflow demos now use the public parent. The interactive demo uses inline feedback for the overflow action.

All five standard Rust checks passed. Chrome verified member geometry, one tab stop, arrow wrapping, End, RTL navigation, keyboard activation of overflow, and caller key cancellation. Avatar, AvatarStatusDot, AvatarGroupOverflow, and showcase navigation regressions passed. Published screenshot parity, other browsers, and manual assistive technology were not verified. The checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8141 node tests/avatar_group.browser.mjs
```
