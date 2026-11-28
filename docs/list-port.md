# Static List checkpoint

`src/list/` ports the static container: native ordered and unordered lists, optional header naming, decimal start, and list-owned spacing and edge geometry. It publishes density, edge, marker style, and divider settings to [ListItem](list-item-port.md); caller-supplied native rows do not consume that context.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `List/List.tsx` (`8fba297c4b35125380a641ec851b1419de72d546`), `ListContext.tsx`, and the List spec (`6da3048131f681eebd74b590df4090496de7ff8d`). The showcase adapts the settings and ordered-steps arrangements with static ListItem rows. Shared counter-column alignment, bleed corner handoff, the legacy edge alias, animated layout, reordering, and the remaining catalog arrangements are open.

The initial checkpoint passed five standard Rust checks and two Chrome suites. SSR verified header naming and native start markup; Chrome checked accessible list names, starting index, list geometry, and showcase navigation.

The pinned source uses a two-track grid for decimal lists and delegates visible markers to ListItem. The Rust List now spans both tracks for every direct `li`, so caller-owned native rows do not auto-place side by side. This does not add visible markers to bare rows or change their native controls, DOM order, or focus ownership. The setup-resources example exercises this composition. Focused regressions cover rendered markup and the bundled placement rule; the browser regression for row geometry and Tab order was added but not run in this follow-up.

Published visual parity and manual assistive technology were not verified. Reorder-mode keyboard behavior still requires the upstream-style reorder handle/session integration, which the existing Rust props do not expose. The checklist remains open.
