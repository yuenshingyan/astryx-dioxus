# StackItem port

StackItem renders one native root and preserves its children and native attributes. It defaults to static sizing, supports fill and cross-axis overrides, and resets both minimum dimensions to zero. The default root is a `div`; `Span` and a caller-rendered root are also supported. The existing Stack fill example uses the public component instead of its temporary wrapper.

Scrollable `div` and `span` items without a caller role gain a named group only when an axis can scroll and content overflows by more than 1px. The caller label wins; `scroll_label` lets callers translate the English fallback “Content.” Other roots and caller roles retain their semantics. Scroll measurement runs in the browser, follows content and size changes, and releases subscriptions on cleanup. No synthetic tab stop is added.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, particularly `Stack/StackItem.tsx`, `Stack/stackItem.stylex.ts`, and `hooks/StackItemScrollRegion.tsx`. Both original templates use horizontal Stack framing without claiming a public HStack port.

Run the focused browser test against the local showcase with `SHOWCASE_URL=http://127.0.0.1:8115 node tests/stack_item.browser.mjs`. The test verifies both patterns, fill geometry, scrolling, and overflow-driven naming. Published screenshot parity, other browser engines, and manual assistive-technology behavior are not certified; the migration checklist remains open.
