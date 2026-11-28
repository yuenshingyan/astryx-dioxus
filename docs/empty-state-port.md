# EmptyState checkpoint

`src/empty_state/` renders a native heading (default `h3`), optional description, decorative artwork, and caller-owned actions. The heading level does not change the title's visual size. Compact presentation reduces padding and stacks the action slot vertically. Absent slots and direct empty text add no wrappers. Following the pinned spec's FR3, icon and action fragments retain their wrappers when the fragment has entries, even if those entries render nothing.

The shared rendered-text announcer reads only the title and description. It suppresses initial-load and hydration announcements and uses the persistent polite live region for later appearances and changes. The visible message is not a live region.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `EmptyState.tsx` (Git blob `5a3b6552dd7e2aee2cc72d3ca8d97fa1df9607d0`), `EmptyState.spec.md` (`cbd47d37037e58331306589b8355a2a313e30d48`), and all four catalog arrangements. Demo illustrations and inline action feedback substitute for the source Heroicons and Toast. The migration checklist remains open.

The focused SSR tests cover direct empty slots and shallow fragment retention, including decorative icon markup, slot order, native headings, and the absence of live-region markup. Dioxus represents a direct false RSX conditional and a fragment containing exactly one canonical placeholder identically; both are treated as absent.

At the original checkpoint, all five standard Rust checks passed. Chrome verified heading markup, compact layout, decorative artwork, and live-region writes for title and description changes and remount. Shared announcement, Layout, and showcase navigation regressions passed. Published screenshots, other browsers, and manual screen-reader speech were not verified.

```sh
SHOWCASE_URL=http://127.0.0.1:8133 node tests/empty_state.browser.mjs
```
