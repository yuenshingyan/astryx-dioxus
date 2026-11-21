# ChatMessageList checkpoint

`src/chat/chat_message_list.rs` provides a focusable, polite transcript log, reflects streaming through `aria-busy`, and supports density-based insets, a row-gap override, top or bottom resting alignment, and an empty state when children are omitted. It does not create a scroll container or imperatively announce changes.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatMessageList.tsx` (`bdfb265682e7d7b5b33aa820b3c29aa5dcc93324`), its spec (`366d05e613cb7c94526f7cc3c0f138340534724d`), and three catalog templates. The examples use shortened copy and substitute plain text for Markdown, attachments, and formatted timestamps. Child density is passed explicitly in the examples: distinguishing omitted from explicit balanced density in ChatMessage requires a compatible API design. ChatLayout registration, async loading of older messages, and its loading announcement are not implemented.

All five standard Rust checks and four Chrome suites passed. Focused verification covered root semantics and accessibility-tree exposure, busy transitions, alignment, insets and gap, and empty content. Published visual parity and spoken announcements were not verified. The checklist remains open.
