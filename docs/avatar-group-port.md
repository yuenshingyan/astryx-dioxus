# AvatarGroup checkpoint

`src/avatar_group/component.rs` renders a named group. The caller selects which avatars and overflow indicator to render. The group publishes size, shape, and quarter-size overlap through a layer-scoped context. Static members add no default tab stops inside the group. Interactive members share one tab stop; Left/Right wrap in reading order, and Home/End jump to the first or last member. The keyboard hint supplements any caller description. The browser controller cleans up its observer and listeners on unmount.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `AvatarGroup/AvatarGroup.tsx` (Git blob `c3d76c1f4da82e26a6ee7f987c379e1fe44727b8`), the component spec, and both showcase templates. The overflow demos now use the public parent. The interactive demo uses inline feedback for the overflow action.

### Interactive-only keyboard hint

FR5 and AR3 require a keyboard hint only while roving applies, with its ID appended after any caller `aria-describedby` (ORD2). Both reference files were compared byte-for-byte with the pinned upstream files; `AvatarGroup.spec.md` is Git blob `754e454f4e745d871efbc20f3fcdcf6f87bb7e14`. Static groups now omit the hint node rather than rendering it hidden. The existing browser observer reports when the roving set becomes nonempty or empty, and the Dioxus group renders or removes the hint. The browser controller only adds the description reference once the hint exists. Unlike the upstream render-time detection, the port's observer can react to membership changes without a parent render.

The focused SSR test passed, including the static group's caller description and absence of hint text. The browser regression covers hint presence, description order, and removal and restoration as members are hidden and shown; it was not run for this follow-up.

At the prior checkpoint, all five standard Rust checks passed. Chrome verified member geometry, one tab stop, arrow wrapping, End, RTL navigation, keyboard activation of overflow, and caller key cancellation. Avatar, AvatarStatusDot, AvatarGroupOverflow, and showcase navigation regressions passed. Published screenshot parity, other browsers, and manual assistive technology remain unverified. The checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8141 node tests/avatar_group.browser.mjs
```
