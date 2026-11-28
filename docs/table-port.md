# Table children-mode checkpoint

`src/table/component.rs` composes caller-supplied native sections in a styled table. Like the pinned Table, it renders its own scroll viewport and observed content box using the shared scroll hook. The hook still manages horizontal overflow and gives the viewport a tab stop only when content overflows. Table styles apply to its own rows and cells, leaving standalone components unstyled. The API exposes density, divider, alignment, and overflow choices, plus striping and hover.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Table/Table.tsx` (`233b1621e1921bfbc17cc6442ac3500deadbe749`), `Table/BaseTable.tsx` (`61f66413f4dce56577a70751c3cb3e38180f332e`), `Table/table.stylex.ts`, and the Table spec (`7bb1254d54cf2f2ace355871635fe60228d38e30`). The showcase adapts TableShowcase and TableGridDividersTable to explicit rows and cells; it does not reproduce data-mode column sizing.

For FR1, the viewport emits `aria-labelledby` when supplied, otherwise the explicit `aria-label`, otherwise `Table`. The native table keeps both caller-provided naming attributes so the browser can apply native accessible-name precedence; it never receives the generic fallback. A native caption remains in place. Both the existing `label`/`labelled_by` props and forwarded ARIA attributes are supported.

The naming follow-up verifies the emitted attributes and native nesting with `cargo test --locked --test table`. It does not recheck the browser accessibility tree or scroll behavior. At the earlier children-mode checkpoint, all five standard Rust checks passed, and Chrome verified compact grid padding and dividers, scrolling, conditional viewport focusability, and table and viewport names in the accessibility tree. The six earlier table component suites and showcase navigation also passed.

Other appearance combinations were not exhaustively compared. Data mode, plugins, virtualization, container bleed and edge insets, cut-text tooltips, remaining catalog templates, published visual parity, and manual assistive technology verification remain open. The migration checklist remains open.
