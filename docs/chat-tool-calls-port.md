# ChatToolCalls checkpoint

`src/chat/chat_tool_calls.rs` displays caller-supplied data without executing tools. It omits empty collections, shows one call inline, and groups multiple calls behind a native disclosure button. The collapsed group previews the latest call; expanded rows show status and metadata, including zero counts. Individual result details have their own disclosure.

Sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Chat/ChatToolCalls.tsx` (`c6135f8d0b6cd3a90c293a1f83aa74af40ff0132`), its spec (`358e893fd09451111c9bc9c5443a0bffc2f14dd6`), and the adapted showcase (`40d4094e243cc1ba98d2c0dfea6a6d75dcd469b0`). The preview uses static sample data and plain preformatted result content.

All five Rust checks and three Chrome suites passed. The focused browser suite verified the latest-call preview, Space activation, zero-count metadata, and result detail disclosure. Animated group transitions, controlled-close focus restoration, exact glyph matching, and published visual parity remain open. The checklist remains open.
