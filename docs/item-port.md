# Static Item checkpoint

`src/item/` ports the presentational row with native div, li, or span roots, marker and leading/trailing slots, and plain or rich inline content. Plain text ellipsizes by default; inline descriptions yield space before labels. Static rows add no focus stop of their own.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Item/Item.tsx` (`33a18f0c2710315510d38ea5edbbb9460bfbc4bc`) and its spec (`97e278feb8560184cb1e7333555f6864039bf981`). The showcase adapts the basic rows and adds an inline metadata probe. Action and link rows, list marker-column integration, delegated activation, swipe behavior, and remaining catalog arrangements are open.

All five standard Rust checks passed. SSR verified root and slot order; Chrome verified compact spacing, inline description truncation, RTL slot order, and List and showcase regressions. Published visual parity and manual assistive technology were not verified. The checklist remains open.
