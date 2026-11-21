# TableHeader checkpoint

`src/table/table_header.rs` renders one native `thead` with the `astryx-table-header` styling target, forwards native attributes and events, and adds no default styles or ARIA roles. It accepts caller-owned header rows and cells.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Table/TableHeader.tsx` (`c147d056f2af298e4a6fcefc897024ef480c1459`), and its spec (`88949434dc2bf1127eaf4e38b8b9d5aa8e82ed0e`). There is no standalone upstream template. The showcase composes the header with a native table, rows, and cells. Parent Table integration and data-mode header generation remain pending.

All five standard Rust checks passed. SSR verified native markup and attribute forwarding. Chrome verified table structure and column-header names in the accessibility tree; showcase navigation also passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
