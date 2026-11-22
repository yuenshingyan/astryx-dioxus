# ResizeHandle keyboard checkpoint

`src/resizable/resize_handle.rs` implements an inline, keyboard-operable separator for a caller-sized region with finite pixel bounds. It exposes the current value and range, uses 10px arrow steps (50px with Shift), and supports Home, End, both axes, and reversed growth. Horizontal RTL mirroring is caller-configured. Invalid bounds disable the handle.

Pinned source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Resizable/ResizeHandle.tsx` (`1217b48d909c2282eb36287730e56960080ac0f8`). The showcase uses caller-managed widths and heights, not `useResizable`.

All five Rust checks and three Chrome suites passed. Chrome verified keyboard steps, bounds, reversed vertical behavior, pane geometry, and retained focus. Pointer dragging, `useResizable` integration, collapse and snap points, overlay positioning, and visual parity remain open. The existing Grid showcase still uses its range input.
