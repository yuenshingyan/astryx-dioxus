# Segmented Control: controlled radio-group checkpoint

`src/segmented_control/` keeps the group, item, and context together. The caller owns the selection. The selected item is the tab stop; Left/Right and Home/End move focus and request selection, skipping disabled items and mirroring horizontal movement in RTL. Up/Down leave focus, selection, and the browser's default action alone. Items support visible or icon-only labels, and a caller click handler can cancel selection. The group supports Hug and Fill layouts and inherits the shared control size.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `SegmentedControl.tsx` (`47a0884d98d7a236387846024d223e4243bdfbe4`), `SegmentedControlItem.tsx` (`fe833a3850a8787ae422f43af838fc6b3db57c50`), and `SegmentedControlContext.ts` (`83c7f45a132cd27c0d3beda19b4f27949916617b`). The primary example retains the pinned Grid/List/Table arrangement. A separate RTL example adapts the item showcase to exercise an icon-only label, disabled skipping, and cancellation.

The keyboard follow-up matches FR2 of the pinned `SegmentedControl.spec.md`
(`02f99605efa824da75065e98e18735ff4575305f`) and the pinned component's
horizontal `useInternalListFocus` configuration.
`node tests/segmented_control_keyboard.mjs` exercises the shipped controller
with DOM stubs in LTR and RTL: vertical keys pass through, and horizontal keys
still skip a disabled segment and request selection without changing the tab
stop while the caller's value is unchanged. Browser integration was not rerun
for this follow-up.

At the original checkpoint, all five Rust checks and three Chrome suites passed. The focused suite verified radio naming, a single tab stop, keyboard selection in LTR and RTL, disabled skipping, and cancellation. Touch drag, expanded hit areas, disabled explanations, selection animation, remaining catalog arrangements, and visual parity remain open. Neither checklist entry is marked complete.
