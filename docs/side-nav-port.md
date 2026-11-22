# SideNav checkpoint

`src/side_nav/component.rs` renders a named expanded rail with header, top content, independently scrolling items, footer, and a small-size context for footer controls. Top and footer content stay outside the scroll area. Empty top and footer regions are omitted.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `SideNav/SideNav.tsx` (`173fdf660a8e28aedb1190196b783f094ca12c8a`), its spec (`2eda2f83e3b10d2e0e97bba957f1f766fc9c7388`), and the adapted showcase (`1ef7a384c06565154f2cc351201e7a687f9f69e9`). The sample substitutes local fragment links and omits the original icons and nested groups.

All five Rust checks and four Chrome suites passed. Chrome verified the named landmark, 260px default width, independent item scrolling, stationary header and footer, and inherited footer sizing. Collapsed and resizable rails, nested navigation, AppShell integration, remaining catalog arrangements, and published visual parity remain open.
