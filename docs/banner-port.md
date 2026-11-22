# Banner checkpoint

`src/banner/` ports the message header: four built-in statuses, validated custom status and container names, card and section treatments, elevation, optional description, icon and action, and local dismissal. An urgent alert is opt-in and applies only to the message, not the controls. Custom statuses do not imply an icon, fill, or announcement.

References at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Banner.tsx` (`566880f8abde52a1a17d2e953b00a44b9899cbcf`), its spec (`21a3e674fd7f9800aa445f8aa4a4080d7a9b1979`), and the adapted Statuses showcase (`a7d513e5d3e613b4da5e2aa46d63a3c0be39d420`). The demo includes a local retry action and an urgent, dismissible error. Supplementary disclosure, rich titles, focus restoration on dismissal, exact glyph matching, and the remaining catalog arrangements are open.

All five Rust checks and two Chrome suites passed. SSR verified custom identifiers and the absence of implicit live semantics. Chrome verified the four status markers, alert scope, action, and keyboard dismissal. Published visual parity and spoken output were not verified; the checklist remains open.
