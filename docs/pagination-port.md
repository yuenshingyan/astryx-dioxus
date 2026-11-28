# Pagination checkpoint

`src/pagination/` provides controlled, one-based page navigation in the Pages and Compact variants. The default Pages window follows FR7: show every page through seven; above seven, show five consecutive pages at the near end and the opposite endpoint, or show both endpoints and the current page with one sibling per side. The switch happens after page 3 and at the third-from-last page. Each omitted range is marked with a non-interactive `…` hidden from assistive technology (FR4). Its default-size styling uses shared element size, text size, and secondary text tokens without adding a public theming target. Numbered controls retain their identity as the range moves. Compact shows the current page and total; both variants use the shared Button for bounded previous and next actions.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Pagination.tsx` (`920b321974ad9ac6c96afc57461a5ff9279ed7fe`), its spec (`35ce251e4c7d83e0349cccaa4382aa1f91a0f3f7`), and `PaginationVariants.tsx` (`29909da985f380d1cdbeeb72cf3ac2ea6a1b2501`). The showcase adapts two variants and uses text labels instead of arrow glyphs for previous and next.

The original checkpoint recorded passing Rust checks and Chrome coverage of landmark and button names, Enter activation, focus retention, and Compact bounds. This follow-up adds SSR coverage for exact window order and decorative ellipses, including the seven/eight-page threshold, both edge transitions, zero pages, and `u32::MAX`. The showcase adds an interactive eight-page example starting at page 1 alongside the existing middle window. No browser or published visual comparison was run for this follow-up.

## Open gaps

- Configurable sibling count is deferred: a new field on the publicly exported `PaginationProps` would break existing Rust struct literals, even with an RSX default. Exposing the option requires an approved API design.
- Count, Dots, Input and None variants, cursor/item-count modes, page-size selection, table integration, and page links remain open.
- The port still clamps the supplied current page (upstream does not), and it lacks configurable stride, first/last shortcuts, page-change announcements and optimistic actions. Upstream arrow controls, full localization, and visual and responsive parity are not verified.
