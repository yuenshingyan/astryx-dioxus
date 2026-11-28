# AppShell desktop checkpoint

AppShell composes the existing Layout, LayoutHeader, and LayoutContent. The desktop shell accepts top navigation, side navigation, and a banner. A native skip link targets the focusable main landmark. Fill mode constrains scrolling to the main content; auto mode lets content grow.

Pinned sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `AppShell/AppShell.tsx` (`f8ae9cb26415e6dbd6969625cc5b81683d05774a`), its spec (`faccadbd7c0646639f36d92e6fe4cc858de58793`), and the adapted showcase (`4ab643229517320ac8fd1915beb3013d81e18555`). The preview uses local links and a bounded desktop frame.

All five Rust checks and four Chrome suites passed. SSR verified distinct skip-link targets and landmark placement. Chrome verified keyboard focus on main and independent scrolling in fill mode. Responsive drawer integration, safe-area handling, elevated corner treatment, remaining catalog arrangements, and visual parity remain open.

## Layout-root inset reset (FR18)

AppShell applies Layout’s reset on its outer element, before the inner Layout can bleed into ancestor padding. The reset clears inherited Section padding propagation but leaves theme tokens intact. Its declarations match the pinned `Layout/layoutReset.ts` and `Layout/overlayPaddingReset.stylex.ts` (blobs `145cc5ba4a5d94380466c1ab6030664845b9bf6d` and `7c4acbe00d29e9f6e7bc31e6f9fc4878b517e179`).

`tests/app_shell.rs` checks root placement; the focused Cargo run stopped before executing tests because the filesystem ran out of space while writing the crate archive. For renderer verification, run `SHOWCASE_URL=<showcase URL> node tests/app_shell.browser.mjs` against the integrator’s showcase. The new case checks zero inner Layout bleed, the nested Section’s 16px default and 20px theme override, and a sibling Section that still inherits 24px. This case has not been run in a browser.
