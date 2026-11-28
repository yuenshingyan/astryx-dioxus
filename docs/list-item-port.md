# Static ListItem checkpoint

`src/list/list_item.rs` composes Item on a native `li` and inherits density, edge treatment, marker style, and dividers from List. Circle, disc, and decimal markers are decorative. Dividers appear between rows, not after the last row. The row adds no tab stop; controls in its slots remain caller-owned.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `List/ListItem.tsx` (`e21993d1e4c317d28a66700cf8b786db10e154c7`) and its spec (`7cf5a2551e2134f8a77383ed06f54c734f00571b`). The List showcase now uses real ListItem rows for settings and ordered steps, with a separate bullet and divider example. Shared counter-column alignment, bleed corner handoff, interactive rows, swipe, reorder, and the remaining catalog arrangements are open.

At the static checkpoint, all five standard Rust checks and four Chrome suites passed. SSR verified inherited settings and slot markup; Chrome verified native list-item semantics, spacious flush geometry, divider placement, and marker styling. Published visual parity and manual assistive technology were not verified.

## Numbered-marker follow-up

Decimal markers now use the body font size and line height, keep the digits and period together, and use the row's logical inline inset before a long counter pushes its label inward. This ports the ordinary counter hang from `ListItem.tsx`, not the shared counter column or Markdown's outside-marker rules. `tests/list_item.browser.mjs` adds a focused LTR/RTL probe for the marker width with and without the inset. It has not been run against a live showcase.

## Activation blocker

The pinned spec (FR1, AR2) assigns the native button/anchor, delegated activation, and disabled/selected semantics to Item. ListItem forwards those props upstream; the local `src/item/component.rs` is presentational and has no such props or native action surface. There is no owner-preserving way to forward them in this ListItem-only change. An Item-owned port must supply the focus target and keep caller-owned controls (including the trailing slot) out of the row action before ListItem can expose that behavior. Static slots remain unchanged. First-line marker alignment, shared counter alignment, bleed corners, task markers, motion and reorder also remain open. No checklist items were changed.
