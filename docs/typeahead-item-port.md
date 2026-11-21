# TypeaheadItem standalone checkpoint

`src/typeahead/typeahead_item.rs` renders a result label with optional artwork and a secondary description. Long lines ellipsize, and disabled styling dims the content without adding option semantics or a tab stop. The parent typeahead owns result identity, selection, and keyboard interaction. A supplied replacement element renders directly and owns its attributes, matching the pinned implementation; the upstream spec records root forwarding on this path as unresolved.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Typeahead/TypeaheadItem.tsx` (`f74950acfbeb68d31c72888f35ddf7e207765007`), its spec (`a88fa7b1e105d82d0169598f685f58d0e8cb94a0`), and both catalog templates. The showcase preserves their people and descriptions and uses Avatar in the second arrangement. It previews results without the unported search host.

All five standard Rust checks passed. SSR verified normal and replacement markup, attribute forwarding on the default root, and absence of added option semantics. Chrome verified narrow ellipsis, RTL artwork placement, dimming, and accessible text. Avatar and showcase navigation regressions passed. Parent integration, published visual parity, and manual assistive technology remain open. The migration checklist stays open.
