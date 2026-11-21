# TableBody checkpoint

`src/table/table_body.rs` renders one native `tbody` with the `astryx-table-body` styling target. It forwards native attributes and events, adds no default styles or ARIA roles, and leaves rows and cells to the caller.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Table/TableBody.tsx` (`b4505f84d07d8478f2c6ab01f7c5dab33a057eb6`) and its spec (`d3f195abadf47f09e5476b32884004dbd5a932e2`). There is no standalone upstream template. The showcase pairs TableBody with TableHeader and TableFooter in native tables. Parent Table integration remains pending.

All five standard Rust checks passed. SSR verified native markup, sibling body sections, and attribute forwarding. Chrome verified section and row order and body row headers in the accessibility tree. TableHeader, TableFooter, and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the migration checklist remains open.
