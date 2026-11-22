# LinkProvider checkpoint

The existing provider in `src/link/link_provider.rs` selects a renderer without adding a DOM wrapper. The nearest provider supplies the default; an explicit renderer wins. Destination validation stays in the shared link boundary. This checkpoint changes no library API.

References at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `LinkProvider.tsx` (`dcff275c9817ff663991c27e1745a003e9f30c0d`), spec (`bc945f81f3659ec1522d804ae6d89fa6af945c70`), and `LinkProviderCustomLink.tsx` (`e3b06f17d5f54d942c73469efc6682ba7030954c`). The local Custom Link arrangement preserves the anchor and intercepts activation for the demo route. It uses an inline output instead of a toast and does not install a router.

All five standard Rust checks and two Chrome suites passed. Chrome verified the native link name and role, Enter interception without a URL change, and focus retention. Real router integrations, modified-click behavior, spoken output, and published visual parity were not verified. The checklist remains open.
