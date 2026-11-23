# InternationalizationProvider checkpoint

The DOM-less provider supplies reactive, plain-text catalogs to its subtree. Lookup walks exact and parent locale tags for overrides first, then supplied messages, then the component's English fallback. Nested providers replace the parent scope. Applications still own DOM language and direction.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `i18n/InternationalizationProvider.tsx` (`dbacf36fb75e9b904cb339c79b3dea4bfb9ae9ac`) and its spec (`16d48a75190d63db0006451200053b115a4af5f0`). The showcase supplies sample Portuguese messages for Pagination's previous and next buttons and switches back to English without resetting the current page.

All five Rust checks and three Chrome suites passed. SSR verified override order, English fallback, and nested scope replacement. Chrome verified live locale changes and preserved pagination state. Rich catalogs, interpolation, direction and calendar context, public consumer hooks, and wider component coverage remain open.
