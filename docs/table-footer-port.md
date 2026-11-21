# TableFooter checkpoint

`src/table/table_footer.rs` renders one native `tfoot` with the `astryx-table-footer` styling target. It forwards native attributes and events and adds no default styles or ARIA roles. Callers supply the footer rows and cells.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Table/TableFooter.tsx` (`e64b31b42eebebc9fc0d463d8752d0f9225d428c`) and its spec (`2c4dd57671672221c1efc3568327f0e050fa23e0`). There is no standalone upstream template. The showcase pairs the footer with the ported TableHeader and native table rows and cells. Parent Table integration remains pending.

All five standard Rust checks passed. SSR verified native markup and attribute forwarding. Chrome verified footer row order and the summary row header in the accessibility tree. TableHeader and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the migration checklist remains open.
