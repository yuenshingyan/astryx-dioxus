# TableSelectionToolbar checkpoint

`src/table/table_selection_toolbar.rs` composes Toolbar and a ghost clear button. With zero selected rows it renders nothing. The caller supplies the count, bulk actions, and a callback that clears the complete selection. The toolbar supports custom count content and labels; it does not own table rows or positioning.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Table/TableSelectionToolbar.tsx` (`eb0af2a9b81e9653cfdf1fe366bd2bfcc1b56b21`) and its spec (`5b7368646f6f0deaed9d32261d6398e85b12be06`). The preview uses native checkboxes and caller-owned selection. Table selection plugin integration, all-matching server-page selection, and focus placement after the clear button unmounts remain open.

All five Rust checks and four Chrome suites passed. Tests covered zero selection, custom count text, accessible toolbar naming, keyboard clear, and Toolbar and Table regressions. Published visual parity and manual assistive technology were not verified. The checklist remains open.
