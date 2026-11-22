# Static ListItem checkpoint

`src/list/list_item.rs` composes Item on a native `li` and inherits density, edge treatment, marker style, and dividers from List. Circle, disc, and decimal markers are decorative. Dividers appear between rows, not after the last row. The row adds no tab stop; controls in its slots remain caller-owned.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `List/ListItem.tsx` (`e21993d1e4c317d28a66700cf8b786db10e154c7`) and its spec (`7cf5a2551e2134f8a77383ed06f54c734f00571b`). The List showcase now uses real ListItem rows for settings and ordered steps, with a separate bullet and divider example. Shared counter-column alignment, bleed corner handoff, interactive rows, swipe, reorder, and the remaining catalog arrangements are open.

All five standard Rust checks and four Chrome suites passed. SSR verified inherited settings and slot markup; Chrome verified native list-item semantics, spacious flush geometry, divider placement, and marker styling. Published visual parity and manual assistive technology were not verified. The checklist remains open.
