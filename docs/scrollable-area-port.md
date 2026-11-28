# ScrollableArea checkpoint

`src/scrollable_area/` renders one named native viewport and one observed normal-flow content box. Logical axis selection follows the viewport's writing mode. The viewport enters the tab order only while a requested axis overflows; when focused content starts fitting, it retains focus until blur. Fitting areas clip by default without establishing a sticky scroll boundary. The content publishes its own logical insets, while full bleed consumes inherited insets at the viewport.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `ScrollableArea.tsx` (Git blob `93e7cf40c0af87ebce68364f38fa4b6f974df5ae`), `useScrollableArea.ts` (`2cfcb802aecb1935e6515e63b7ed8248adabd5cf`), and the six catalog templates. The showcase adapts all six arrangements with simplified file content and native CSS sticky headers. The browser hook is internal to this port; standalone hook consumers and the upstream scroll-owner registry are not implemented.

At the initial checkpoint, all five standard Rust checks passed. Chrome verified overflow and edge flags, RTL and vertical writing-mode mapping, sticky containment CSS, overscroll policy CSS, and focus retention on overflow loss. LayoutContent, StackItem, and showcase navigation regression suites passed. Actual gesture chaining, sticky positioning during scroll, other browsers, published screenshots, and manual assistive technology remain unverified. The migration checklist stays open.

## Scroll-state styling follow-up

Following FR4 in the pinned `ScrollableArea.spec.md`, only the viewport opts into `container-type: scroll-state`, behind the same `@supports` condition as `ScrollableArea.tsx`. This lets supporting browsers use descendant scroll-state queries without making them a dependency of native scrolling. Hook-owned edge state, overflow rules, keyboard focus, and existing full-bleed behavior are unchanged.

`cargo test --locked --test scrollable_area` checks the guard in the public CSS bundle and retains the SSR viewport test. The browser probe now also checks the computed container type on the viewport and content box; it was not run for this follow-up.

```sh
SHOWCASE_URL=http://127.0.0.1:8142 node tests/scrollable_area.browser.mjs
```
