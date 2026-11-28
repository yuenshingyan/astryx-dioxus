# Banner checkpoint

`src/banner/` ports the message header: four built-in statuses, validated custom status and container names, card and section treatments, elevation, optional description, icon and action, and local dismissal. An urgent alert is opt-in and applies only to the message, not the controls. Custom statuses do not imply an icon, fill, or announcement.

References at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Banner.tsx` (`566880f8abde52a1a17d2e953b00a44b9899cbcf`), its spec (`21a3e674fd7f9800aa445f8aa4a4080d7a9b1979`), and the adapted Statuses showcase (`a7d513e5d3e613b4da5e2aa46d63a3c0be39d420`). The demo includes a local retry action and an urgent, dismissible error. Supplementary disclosure, rich titles, exact glyph matching, and the remaining catalog arrangements are open.

## Dismissal focus handoff

On the web, Banner records the last external `HTMLElement` before focus enters (`focusin`'s `relatedTarget`) or before a pointer press inside the Banner moves focus (`pointerdown`'s active element). It captures both events on the Banner frame. A missing candidate, `body`, a non-HTML element, or a move within the Banner does not overwrite the origin. Dismissal snapshots the origin, queues removal, invokes `on_dismiss`, then focuses the origin synchronously if it is still connected. There is no fallback target. The listeners are removed on dismissal or unmount. Banner's content props and API are unchanged.

Prior Chrome verification covered the four status markers, alert scope, action, and keyboard dismissal. The focused SSR test passed again; it covers custom identifiers and default live-region semantics, not DOM focus. `tests/banner.browser.mjs` now covers keyboard focus return before removal, retention across internal focus moves, pointer capture, and skipping a detached origin; those cases were added but not run for this follow-up. WASM compilation, published visual parity, and spoken output were not verified; the checklist remains open.
