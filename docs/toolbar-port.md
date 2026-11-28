# Toolbar checkpoint

Toolbar composes Section and SizeProvider around a named action row. It supports two or three lanes, divider edges, gap overrides, and horizontal or vertical orientation. A private list-focus controller provides one roving tab stop for ordinary controls, reverses horizontal arrows in RTL, and leaves editing keys to inputs and nested widgets.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Toolbar/Toolbar.tsx` (`c6b70196c044a774fa59361fa339bda89e15f806`) and the Toolbar spec (`172882a94e095eec54a839114315a87077cb1084`). The showcase covers three lanes, a native filter input, and vertical navigation. Tab rail and edge compensation, shared keyboard hints, caret-edge handoff, remaining catalog arrangements, published visual parity, and manual assistive technology verification remain open.

At the original checkpoint, all five standard Rust checks passed. Chrome verified the accessible toolbar name, roving tab stop, RTL arrow reversal, vertical navigation, and showcase navigation. The checklist remains open.

## FR4 follow-up: Home and End

The existing controller already moves Home and End to the first and last enabled controls. `tests/toolbar.browser.mjs` now disables each endpoint in turn and checks that the corresponding key moves focus past it and makes the destination the only enabled tab stop. This is regression coverage, not a change to focus behavior. The new browser assertions were syntax-checked but not run in this follow-up. The focused Rust test could not run because the build ran out of disk space.

The port still leaves editing keys with text inputs rather than handing off at caret edges. Focus on an owned control, including pointer-driven focus, can move the port’s tab stop; the pinned spec records that a pointer click does not move the stop upstream. Neither difference is changed or covered by this follow-up. Any follow-up that changes shared focus policy or ownership needs separate approval.
