# AvatarStatusDot migration audit

## Pinned inputs

Audited Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`. The supplied `AvatarStatusDot.tsx` (`8f4177c29864837815fbec80ac2f53f08d1a276a`) and spec (`4e202d1434ff13f84a4542df7b25c0468333e64c`) match the pinned Git tree. Also checked the core doc, unit tests, stories, Avatar implementation/tests and context definitions, BaseProps, slot/merge/theme utilities, Layout/HStack/Stack, and shared tokens against that tree. The CLI templates were verified as source, not as published screenshots:

| Template | TSX blob | Metadata blob |
| --- | --- | --- |
| AvatarStatusDotShowcase | `26f4a69fbcb9bc31527abb2feb3a35c35fb8e0d1` | `6ff3cbe2f8a5d8d7523dc2b426b4cd5a42f74912` |
| AvatarStatusDotVariants | `ad3b7c2aad407db279603af56ca4d97dfb2ad6e8` | `75fc8ee7e7a2ef671cf3482ad7ff6c25b8a505d0` |

## Checklist evidence

- **Size and icon:** public Avatar SSR exercises the 10/20/32px tiers with 1/2/4px surface borders, 12/18px replacement boxes, and the built-in mark at the small tier. A direct empty RSX slot now retains the built-in mark on larger dots too; an icon component that itself renders nothing still replaces it, as in the source. Existing colocated coverage checks the layer reset to the standalone 36px avatar context. The showcase includes a separate three-tier icon example.
- **Showcase and Variants:** both render through public Avatar's status slot, with the original names, labels, order, XL sizes, centered HStack and shared spacing step 4 (1rem). Focused SSR renders the actual showcase module. These checks close the two template composition items, not rendered visual parity or the dependencies' own migrations.
- **Built-in variants (open):** the port uses the matching shared roles: success/on-success, surface/secondary-text with a ring, and error/on-error with a round-capped minus. Source and port agree on the 1/1.5/2px stroke ladder and 75% minus span. Earlier Chrome evidence covers Neutral-theme standalone light/dark paint and small-tier geometry. Rendered comparison with the pinned original still needs to verify the filled/ring/barred distinction across tiers without relying on color, including a forced-colors check. Caller-supplied replacement marks must differ by status for the same distinction to hold.
- **Accessible labels (open):** SSR verifies the standalone image role and caller label, no generated role/name for empty label, and hidden icon wrappers. Earlier Chrome evidence covers standalone AX names and wrapped Avatar label updates/removal (DOM). Avatar-owned label reporting in Rust uses an effect plus cleanup; it does not compose the name in SSR without explicit `status_label`, and changes the Avatar name via a signal-driven rerender. The spec expects direct-label render coverage and wrapped reporting before paint without another render. Resolving that gap needs Avatar-owner work; fresh AX-tree evidence for the templates and manual screen-reader checks for composed names, changes/removal, and hidden artwork are outstanding.

## Other contract boundaries

The existing Rust API uses CSS class/style, native root attributes and `onmounted` rather than React `xstyle` and refs. SSR checks root identity, style/class merge, label precedence and reflected variant. Live `onmounted` forwarding is not verified here. The spec's FR6 gap remains: an unmatched custom variant emits its identifier and geometry without a safe baseline plate or mark. The port retains the pinned behavior; choosing a fallback needs a theming decision, not an invented paint rule. Avatar placement, identity and interactivity stay Avatar-owned.

The parent remains open. `cargo fmt --check`, bundle verification and `cargo test --locked --test avatar_status_dot` (via the assigned cargo slot; three tests) passed. Full Rust/Clippy/Wasm checks, the expanded browser suite, pinned visual comparison and manual AT were not run in this worktree. No browser or preview was started. On an integrator-owned full showcase preview, run the two suites under the shared browser lock:

```sh
SHOWCASE_URL=http://127.0.0.1:8123 node tests/avatar_status_dot.browser.mjs
SHOWCASE_URL=http://127.0.0.1:8123 node tests/avatar.browser.mjs
```
