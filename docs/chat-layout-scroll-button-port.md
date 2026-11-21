# ChatLayoutScrollButton checkpoint

`src/chat/chat_layout_scroll_button.rs` renders a centered pill with a native Button. The icon-only mode has a caller-localizable name, and a visible label expands the pill. The outer wrapper receives caller attributes; the inner pill owns surface styling and becomes inert immediately when hidden. The caller owns visibility and scroll behavior.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatLayoutScrollButton.tsx` (`e8c3818d9b7c3e2a31537c4dc627f1231e7c3d42`), its spec (`52f5a83afc86eb102da0da4e5a27e10cf64c2f0f`), and both catalog templates. The examples use a local chevron and standalone visibility control. ChatLayout scroll and focus handoff, shared icon and translation registry integration, and published visual parity remain open.

All five standard Rust checks and three Chrome suites passed. Focused checks covered pill dimensions, accessible names, Enter activation, focus exclusion during exit, and reduced motion. Manual assistive technology was not verified. The checklist remains open.
