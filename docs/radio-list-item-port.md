# RadioListItem checkpoint

RadioListItem composes Item with a native radio and decorative RadioIndicator. RadioList supplies the controlled value, shared form name, control size, and disabled and required settings. Caller-owned native radios remain supported. The input references the row's visible label and description and owns keyboard focus.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `RadioList/RadioListItem.tsx` (`5a012a65cc6e39e8106a2086488058c84c9226cc`) and its spec (`7ae4ab8b2ebda5bcc3eb0a4fe0b495ff3fa4d9ad`). The horizontal showcase now uses controlled RadioListItem rows. Whole-row click delegation, disabled-reason help, touch press geometry, and the remaining catalog arrangements are open.

All five standard Rust checks and four Chrome suites passed. Chrome verified controlled arrow-key selection, retained focus, blocked disabled activation, and accessible naming and description from visible content. Published visual parity and spoken output were not verified. The checklist remains open.
