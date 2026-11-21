# ChatMessageMetadata checkpoint

`src/chat/chat_message_metadata.rs` composes optional timestamp, footer, and delivery status, with dots only between present slots. User rows reverse visual order without changing DOM order. The status caption is visible text with a decorative icon; it does not add a live region or the upstream generic-span `aria-label`, which the pinned spec identifies as unsupported. Sending pulses in CSS and respects reduced motion. Callers can supply a localized caption.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatMessageMetadata.tsx` (`35b1a4a85d4e9278b6be80f01186b6166f77f5e7`), its spec (`fda4157c86bf01f5506af38d424030878542c00e`), and four catalog templates. The previews use plain message containers, static native time elements, and local substitute glyphs. ChatMessage sender context, Timestamp formatting, and shared icon and translation integration remain pending.

All five standard Rust checks passed. SSR covered empty slots, zero text, separators, and DOM order. Chrome covered the five captions, LTR/RTL user-row geometry, error color, reduced motion, and accessibility-tree exposure of the status text and caller footer button. ChatSendButton, ChatSystemMessage, and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
