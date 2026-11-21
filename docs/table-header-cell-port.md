# TableHeaderCell standalone checkpoint

`src/table/table_header_cell.rs` renders a native `th` with the `astryx-table-header-cell` styling target. It forwards native attributes and events, shares `TableCellScope` with TableCell, and adds no default styles or ARIA role.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Table/TableHeaderCell.tsx` (`50d7054158f35ffb5633af1ca7103c929cc581f8`) and its spec (`cb2a9f35bec722dec14d42e454334adfe6e63cac`). There is no standalone catalog template. The existing native-table showcase demonstrates row and column headers. Table-owned density, dividers, overflow and cut-text tooltips, and context menu integration remain pending.

All five standard Rust checks passed. SSR verified the native markup, all four scope values, and caller attribute forwarding. Chrome verified native scope and row and column header roles in the accessibility tree. TableCell, TableHeader, and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
