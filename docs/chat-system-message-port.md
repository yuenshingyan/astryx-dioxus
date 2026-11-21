# ChatSystemMessage checkpoint

`src/chat/chat_system_message.rs` renders either a centered notice with an optional leading icon or a date marker composed with Divider. Neither variant creates a live region or focus target. Divider owns separator semantics and naming; the enclosing transcript owns announcements and scrolling.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatSystemMessage.tsx` (`d34edb3d0843d36dbf224295feb6aa327eb95c1f`), its spec (`ac8854e9fb904ecd761dbbc55c00af29010328c2`), and the four catalog templates. The showcase retains the 20 rows across the four arrangements, using static containers instead of ChatMessageList and local substitute glyphs.

All five standard Rust checks passed. SSR verified icon ordering and omission, root attributes, and delegated separator markup. Chrome verified a named separator in the accessibility tree, absence of local live/focus attributes, and narrow RTL wrapping without shrinking the icon. Divider and showcase navigation regressions passed. ChatMessageList integration, published visual parity, and spoken announcements were not verified. The checklist remains open.
