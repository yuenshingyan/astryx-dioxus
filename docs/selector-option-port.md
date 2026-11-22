# SelectorOption checkpoint

SelectorOption composes the static Item row without adding a wrapper, tab stop, or selection semantics. The row has no default inset or radius and can shrink in a narrow host. Callers supply the label, optional description, leading artwork, and trailing content.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Selector/SelectorOption.tsx` (`756ef9631c19b6becd3c76f12b5604d3d8de5f6e`), its spec (`43b75021ddb74c8c9b58f452cf0cb9fd3c6f0b76`), and the three catalog templates. The previews show row content, not a Selector popup. Host-enforced inline layout, selected-value integration, semantic icon lookup, and supported DOM/ref passthrough remain open.

All five Rust checks and three Chrome suites passed. SSR verified the single Item root and slot order. Chrome checked zero inset, stacked and inline layout, description ellipsis, trailing content, and Item/showcase regressions. Published visual parity and manual assistive technology were not verified. The checklist remains open.
