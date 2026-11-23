# Theme: stylesheet-backed scopes

`src/theme/component.rs` adds a `display: contents` theme scope with a validated stylesheet name and System, Light, or Dark mode. A System scope inherits the surrounding scheme unless its nearest Theme forces a scheme, in which case it follows the browser preference. This checkpoint does not modify the document root.

Pinned Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `theme/Theme.tsx` (`635f16a1b35db51464f6b8975aaee4fca10f23a2`), its spec (`0c09d627cb4521e7c153ed5c6a33fb7f70e2446c`), and the Warm and Forest showcase (`90697dc4055f1c83a4d45c31bd3ca2d9e10d9b6e`). The preview supplies the pinned palette values through local CSS instead of `defineTheme`.

All five Rust checks and three Chrome suites passed. SSR verified scope markers; Chrome verified palette isolation between sibling scopes, reactive mode changes, and a nested System scope inside Dark. `defineTheme`, stylesheet registration and injection, root and portal synchronization, cross-theme selector isolation, and published visual parity remain open.
