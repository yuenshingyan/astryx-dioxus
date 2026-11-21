# TableCell standalone checkpoint

`src/table/table_cell.rs` renders a native `td` with the `astryx-table-cell` styling target and no default styles or ARIA role. It forwards native attributes and events, including spans, and exposes the header references and finite scope values as props. Use a native header cell when header semantics are needed.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Table/TableCell.tsx` (`22485214073b2fe60b6729f8fa0c0e8b24039482`) and its spec (`7997fb4e54a8607be836d686896fb20f932b6af4`). There is no standalone catalog template. The existing project-hours showcase demonstrates a data cell with explicit row and column header references. Table-owned density, alignment, overflow, dividers, container insets, and context menus remain pending.

All five standard Rust checks passed. SSR verified native markup, spans, and attribute forwarding. Chrome verified native cell semantics and DOM header references; TableBody, TableHeader, TableFooter, and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
