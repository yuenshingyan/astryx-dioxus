# Tree List: keyboard navigation checkpoint

`src/tree_list/` renders a named tree with recursive items, local branch expansion, and caller-owned selection. Collapsed children stay mounted but are hidden. A single roving tab stop moves among visible items; horizontal branch keys mirror in RTL. The Project files showcase uses the pinned hierarchy and updates selection locally without opening files.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TreeList/TreeList.tsx` (`c277c6a26efd62cd5b54ff243b8827fafc48f455`), `TreeListItem.tsx` (`55481b0a3e4648f8c74782e0bb5f1403a2ab1719`), `TreeListTypes.ts` (`da623b47710d33093e069bd228fc98b5d2ee1a32`), the spec (`6eda358e01ef92b74e78ff98de6bafbda524438a`), and the showcase (`799f6fb3b5a1a16c45cb89fd120c61300f44d392`).

All five Rust checks and the Tree List, List, and showcase-navigation Chrome suites passed. The focused suite checked the accessible tree name, exclusion of collapsed items, branch expansion, keyboard movement and selection, RTL, and the single tab stop. Link rows, item slots, typeahead, animation, density and guide variants, and visual parity remain open. The checklist remains open.
