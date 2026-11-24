# Layer Provider: viewport sizing checkpoint

`src/layer/layer_provider.rs` supplies logical viewport insets without adding a DOM wrapper. An omitted inset inherits the parent declaration; an explicit declaration replaces it. `Popover` consumes the nearest declaration and subtracts its insets from the viewport size budget. This limits the popup's dimensions; it does not reposition the popup away from an individual reserved edge.

Pinned Meta Astryx source at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Layer/LayerProvider.tsx` (`df0ad8c6b64e89dafa5324afafcef693aeb4d4cb`) and its spec (`f3b269d45be4d3d5ea91448e5abf5fc2b6f58500`). The local showcase compares inherited, overridden, and unprovided insets.

The CSS bundle check, all five Rust checks, and four Chrome suites passed. The focused suite checked nested inheritance and replacement, sibling isolation, reactive clearing, and the Popover width budget. Edge-aware placement, additional layer consumers, and the lazy toast host and toast configuration remain open. The checklist remains open.
