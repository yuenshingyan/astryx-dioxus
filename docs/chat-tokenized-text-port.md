# ChatTokenizedText checkpoint

The component replaces literal token values in serialized plain text with inline Badge or caller-rendered content. Empty values are ignored. Matching is left to right; same-offset matches use definition order, and the last definition of an identical value controls its rendering. All other text remains plain and escaped. Dioxus uses a `text` prop because its `children` prop is reserved for elements.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatTokenizedText.tsx` (`0062f374e8c967e1d97ed083d8d210d6601cb36f`), its spec (`802a7c46a0712e0f16f8e4fff7393c5a7a3bcf53`), and all three catalog templates. The previews compose the ported ChatMessage and ChatMessageBubble. ChatComposerInput integration remains pending.

All five standard Rust checks and three Chrome suites passed. SSR covers overlapping literals, duplicate definitions, Unicode, custom replacements, and escaping. Chrome checks the three catalog arrangements, text order, badge variants, and line-height geometry. Published visual parity and manual assistive technology were not verified. The checklist remains open.
