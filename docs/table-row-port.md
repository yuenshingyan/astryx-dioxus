# TableRow standalone checkpoint

`src/table/table_row.rs` renders one native `tr` with the `astryx-table-row` styling target. It forwards native attributes and events and adds no default styles or ARIA roles. Place rows inside a header, body, or footer so the browser does not insert an implicit section when parsing server output.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Table/TableRow.tsx` (`019a649bfb0f0757d4f5fd9470c36ebc8010db1f`) and its spec (`b9658117922e0358a7a14f150723fa7092d1ed5a`). There is no standalone catalog template. The native-table showcase uses TableRow in all three sections. Table-owned stripe, hover, divider, glide, and selection integration remain pending.

All five standard Rust checks passed. SSR verified native markup, section nesting, and attribute forwarding. Chrome verified section order and native row semantics in the accessibility tree. The five earlier table component suites and showcase navigation passed. Published visual parity and manual assistive technology were not verified; the migration checklist remains open.
