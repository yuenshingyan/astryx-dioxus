# Outline checkpoint

`src/outline/` renders a named table-of-contents landmark with native fragment links and a caller-controlled current location. It preserves document order, percent-encodes heading IDs, inherits the shared link renderer, and uses logical indentation at both densities.

Sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Outline/Outline.tsx` (`70646fea84e431aacb1d25e1f58b7393048472c1`), its spec (`4a6b29a13ae78eeb7c91cc47bddb1ec6aab9ddbb`), and the adapted showcase (`e3f3a5ca2fdfc15247358cf7d88a2e973c8e4fc5`). The showcase uses a fixed current item and native fragment navigation rather than scroll-spy. The current-location marker is static rather than animated.

All five Rust checks and three Chrome suites passed. SSR verified the landmark, encoded fragments, and current-location markup. Chrome verified accessible naming, indentation, and Enter navigation. Scroll-spy, custom scroll containers and navigation callbacks, remaining catalog arrangements, and published visual parity remain open.
