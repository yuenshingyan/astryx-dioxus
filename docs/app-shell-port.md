# AppShell desktop checkpoint

AppShell composes the existing Layout, LayoutHeader, and LayoutContent. The desktop shell accepts top navigation, side navigation, and a banner. A native skip link targets the focusable main landmark. Fill mode constrains scrolling to the main content; auto mode lets content grow.

Pinned sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `AppShell/AppShell.tsx` (`f8ae9cb26415e6dbd6969625cc5b81683d05774a`), its spec (`faccadbd7c0646639f36d92e6fe4cc858de58793`), and the adapted showcase (`4ab643229517320ac8fd1915beb3013d81e18555`). The preview uses local links and a bounded desktop frame.

All five Rust checks and four Chrome suites passed. SSR verified distinct skip-link targets and landmark placement. Chrome verified keyboard focus on main and independent scrolling in fill mode. Responsive drawer integration, safe-area handling, elevated corner treatment, remaining catalog arrangements, and visual parity remain open.
