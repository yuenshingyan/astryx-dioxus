# Static List checkpoint

`src/list/` ports the static container: native ordered and unordered lists, optional header naming, decimal start, and list-owned spacing and edge geometry. It publishes density, edge, marker style, and divider settings to [ListItem](list-item-port.md); caller-supplied native rows do not consume that context.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `List/List.tsx` (`8fba297c4b35125380a641ec851b1419de72d546`), `ListContext.tsx`, and the List spec (`6da3048131f681eebd74b590df4090496de7ff8d`). The showcase adapts the settings and ordered-steps arrangements with static ListItem rows. Shared counter-column alignment, bleed corner handoff, the legacy edge alias, animated layout, reordering, and the remaining catalog arrangements are open.

All five standard Rust checks and two Chrome suites passed. SSR verified header naming and native start markup; Chrome checked accessible list names, starting index, list geometry, and showcase navigation. Published visual parity and manual assistive technology were not verified. The checklist remains open.
