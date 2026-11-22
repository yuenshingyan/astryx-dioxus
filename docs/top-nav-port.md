# TopNav checkpoint

`src/top_nav/component.rs` composes a named default-mode navigation landmark with heading, start, center, and end slots. An explicit start slot takes precedence over the children alias. The region layout wraps when constrained and centers the middle slot when all regions fit.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TopNav/TopNav.tsx` (`4c6626fdd8f206a6f4a1bd4e21f9c3c6bd76e788`), its spec (`9161236db53bf87b07e2f4b5ecf07cc6fb2bd9f7`), and the adapted showcase (`504ad729bdbe1d4a47001aa3f65ef4d0b9309b1a`). The showcase uses local fragment links and text buttons in place of utility icons, plus an RTL center-slot example.

All five Rust checks and four Chrome suites passed. SSR verified default naming and start-slot precedence. Chrome verified landmark naming, RTL centering, and narrow wrapping, with regressions for TopNavItem, TopNavHeading, and showcase navigation. Mobile bar and drawer modes, slot-dependent menu integration, and published visual parity remain open.
