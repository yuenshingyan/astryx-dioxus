# Toolbar checkpoint

Toolbar composes Section and SizeProvider around a named action row. It supports two or three lanes, divider edges, gap overrides, and horizontal or vertical orientation. A private list-focus controller provides one roving tab stop for ordinary controls, reverses horizontal arrows in RTL, and leaves editing keys to inputs and nested widgets.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Toolbar/Toolbar.tsx` (`c6b70196c044a774fa59361fa339bda89e15f806`) and the Toolbar spec (`172882a94e095eec54a839114315a87077cb1084`). The showcase covers three lanes, a native filter input, and vertical navigation. Tab rail and edge compensation, shared keyboard hints, caret-edge handoff, remaining catalog arrangements, published visual parity, and manual assistive technology verification remain open.

All five standard Rust checks passed. Chrome verified the accessible toolbar name, roving tab stop, RTL arrow reversal, vertical navigation, and showcase navigation. The checklist remains open.
