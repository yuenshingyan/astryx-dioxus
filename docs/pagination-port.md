# Pagination checkpoint

`src/pagination/` provides controlled, one-based page navigation in the Pages and Compact variants. Pages keeps both endpoints and the current page visible with one sibling on each side where space in the sequence permits. Numbered controls retain their identity as the range moves. Compact shows the current page and total; both variants use the shared Button for bounded previous and next actions.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Pagination.tsx` (`920b321974ad9ac6c96afc57461a5ff9279ed7fe`), its spec (`35ce251e4c7d83e0349cccaa4382aa1f91a0f3f7`), and `PaginationVariants.tsx` (`29909da985f380d1cdbeeb72cf3ac2ea6a1b2501`). The showcase adapts two variants and uses text labels instead of arrow glyphs for previous and next.

All five Rust checks and two Chrome suites passed. SSR verified the current page and window endpoints; Chrome verified landmark and button names, Enter activation, focus retention, and compact bounds. Other variants, page-size selection, table integration, and published visual parity remain open.
