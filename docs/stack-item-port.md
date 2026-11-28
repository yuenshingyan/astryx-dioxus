# StackItem port

## Pinned authority

Source: [Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e). The supplied `StackItem.tsx` matches the pinned blob `adf09881d759ee485b76a2b90c5d0e3184b97a0e`. There is no standalone spec; `docs/specs/components/Stack/Stack.spec.md` (blob `de29630eed62a6ad447300993c284dc225e79b81`) covers StackItem in FR6/FR7, PR1 and AR1/AR2. The family English doc, StyleX utility, scroll region and overflow hooks, upstream tests, template bodies and metadata, and the Stack/HStack/VStack, Card and Text sources were checked against the pinned Git tree.

| Template | Pinned TSX blob | Composition |
| --- | --- | --- |
| `StackItemFill` | `4cca4cf1853b2cdbe0f6d7f9a8ba4cab73ca3894` | Center-aligned HStack, gap 3, width 100%, cap 400px, static and fill Cards. The static neighbor is intrinsic, not an explicit pixel width. |
| `StackItemShowcase` | `027a116639cb76ade8320969212752730dfe103b` | Same frame, cap 500px, static/fill/static Cards. Both templates use padding 3 and supporting secondary Text. |

The shared showcase now uses HStack for both templates and includes the source's gap-2 Logo/Content/Actions example. A separate VStack probe covers an unwrapped child and all four cross-axis overrides against a centered parent. This exercises existing dependencies without asserting their standalone migrations are complete.

## Contract and verification

- The base class resets both minimum dimensions; omitted size and `Static` neither grow nor shrink. `Fill` sets growth to 1 without forcing a basis. Alignment classes map to `flex-start`, `center`, `flex-end`, and `stretch`. None of these rules need a separate StackItem token.
- `is_scrollable` selects `overflow: auto`. Only potentially scrolling built-in `div`/`span` roots without a caller role invoke browser measurement. The shared hook checks both scroll-capable axes for overflow beyond 1px, follows content and size changes, and removes subscriptions on cleanup. No synthetic tab stop is added. The Rust implementation registers its shared hook even on static items, but does not install observers there; this is not the literal hook-free React PR1 implementation.
- The default `div`, `Span`, and `Custom` keep one caller-facing root and optional children. `Custom` must forward the supplied attributes and children. Native CSS class/style and mounted-event attributes are the adapters for StyleX/className/style and React `ref`. SSR checks root and attribute passthrough; it cannot show that mounted data points to the live root.
- Explicit `aria-label` wins; otherwise `scroll_label` supplies a caller-localized value or defaults to English “Content.” The upstream fallback reads the active catalog instead. The current Rust prop is a public `String` already defaulted to `"Content"`, so changing behavior based solely on that string would also change explicitly provided English labels. Omitted-versus-explicit locale fallback needs a compatible API design or approval; it is not claimed here.

The original port ([PR #23](https://github.com/yuenshingyan/astryx-dioxus/pull/23)) reported a focused Chrome pass for both template layouts and dynamic vertical scrolling with the original observer. The [Stack comparison](stack-port.md) covers separate Stack Fill Item captures, not screenshots of these two StackItem templates. The focused SSR suite covers the three static/fill showcase sequences, alignment choices, and caller label without fabricated SSR overflow semantics. In the allotted Cargo-slot run, four tests passed. The showcase assertion failed because it sliced away the opening tag containing `max-width`; the slice and a misplaced RSX key were corrected after that run. The corrected showcase test has not been rerun.

Integrator follow-up (with the shared CDP lock and a separately served showcase):

```sh
SHOWCASE_URL=http://127.0.0.1:8115 node tests/stack_item.browser.mjs
```

The expanded browser suite is syntax-checked, not run in this worktree. It adds actual cross-axis geometry, horizontal span scroll with the caller label, vertical overflow transitions, accessibility-tree group names and a scrolling semantic article. Still needed: live mounted-root/ref evidence, current shared-observer regression checks including cleanup, pinned visual comparisons of both named templates with the actual dependencies, and manual keyboard/screen-reader checks of scroll access and announcements (including a non-Chromium engine). The checklist parent remains open.
