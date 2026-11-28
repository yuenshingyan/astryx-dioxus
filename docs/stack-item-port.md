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

## Verified evidence

- The offline fixture hashes match the pinned source, family spec, and both template bodies. Source comparison confirms the HStack frame, gap, alignment, width caps, Card padding, and supporting secondary Text choices. This compares composition, not rendered appearance.
- All five focused SSR tests passed in the assigned offline Cargo slot (`test --locked --offline --test stack_item`), including the corrected showcase test. They cover static/fill sequences, width caps, alignment classes, root and attribute passthrough, and caller role/label without fabricated overflow semantics.
- The integrator ran `tests/stack_item.browser.mjs` successfully in Chrome on merged [PR #241](https://github.com/yuenshingyan/astryx-dioxus/pull/241), which also passed full CI. The suite verifies cross-axis geometry for all four alignment choices against a centered VStack, the Logo/Content/Actions example, and remaining-space geometry in both named rows. With the current shared observer, it verifies programmatic horizontal span and vertical div scrolling, the div losing and regaining overflow semantics, the span's caller-provided AX name and the div's caller-supplied fallback AX name, and a scrolling custom article without injected role or label. Chrome AX output is not screen-reader speech, and programmatic scrolling is not keyboard access.

## Remaining gates

- **Scroll lifecycle and live root (FR6/FR7):** The Chrome case changes content height without unmounting or disabling measurement. A coordinated live probe must verify observer/listener release on disable and unmount and confirm that the mounted callback receives the same DOM node that owns the caller attributes and children (including a forwarding custom root). Source cleanup code and SSR markup are not live evidence.
- **Naming and keyboard access (AR1/AR2):** A shared catalog integration and a compatible way to distinguish omitted `scroll_label` from explicit `"Content"` are still needed for the automatic translated fallback. Manual checks must record whether Tab reaches an overflowing root and whether keys scroll it, and what the screen reader says for the named group, including in a non-Chromium engine. Where the browser does not make the root a tab stop, the pinned spec calls for a focusable child or `ScrollableArea`, not an injected `tabindex`.
- **Static path (PR1):** No measurement subscriptions on static items by source inspection, but the Rust path still uses the shared hook. Literal hook-free parity is not verified; the Dioxus adaptation needs explicit disposition before claiming PR1.
- **Named-template appearance:** Compare matched upstream and Rust renders of `StackItemFill` and `StackItemShowcase` with their actual HStack/Card/Text dependencies. The available [Stack comparison](stack-port.md) captures a different template (`StackFillItem`); neither those rasters nor PR #241's Chrome geometry close this gate.

The checklist parent remains open. This follow-up did not change runtime behavior or start a preview or browser.
