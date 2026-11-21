# AvatarStatusDot checkpoint

The public dot renders independently of Avatar. A standalone dot uses Avatar's 36px default size context. The private Avatar-owned context resolves 10px, 20px, and 32px dot tiers, with 1px, 2px, and 4px borders; `LayerContentBoundary` resets it. A supplied icon replaces the built-in mark only in the larger tiers. Success uses a filled plate, Neutral a ring, and Error a minus. A custom theme identifier with no rule keeps its geometry and has no built-in paint or mark. A supplied nonempty label names the root as an image; marks are decorative.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Avatar/AvatarStatusDot.tsx` (Git blob `8f4177c29864837815fbec80ac2f53f08d1a276a`). The source, spec, unit tests, stories, and both CLI templates were checked against the pinned Git tree.

The Showcase and Variants templates now use the public [Avatar](avatar-port.md). The standalone states remain available separately. The browser suite for Avatar verifies status-label updates and removal through a wrapper component. The migration checklist remains open.

All five standard Rust checks passed. SSR tests verify size tiers, icon precedence, and the layer reset. Chrome verified standalone geometry, Neutral light/dark paint, mark shapes, accessible names, and showcase navigation. Published screenshot parity, other browsers, and manual assistive technology were not tested.

```sh
SHOWCASE_URL=http://127.0.0.1:8123 node tests/avatar_status_dot.browser.mjs
```
