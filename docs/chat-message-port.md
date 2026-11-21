# ChatMessage checkpoint

ChatMessage owns the named article, sender alignment, three density settings, and optional avatar, name, and metadata slots. System messages suppress those slots. The visible name labels the article when present, with a caller-localizable sender fallback. ChatMessageMetadata inherits the nearest message sender; its standalone `is_user` prop remains available.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatMessage.tsx` (`27f90727c6b1ab9fe1221c0ac3efeca7ec75ccb6`), its spec (`4b87dad31179f3f94bd211ad434bd8a177435e09`), and four catalog templates. The previews use static transcript containers, [ChatMessageBubble](chat-message-bubble-port.md), and shortened sample copy. ChatMessageList density inheritance and translation-provider support remain pending.

All five standard Rust checks passed. Chrome verified accessible article names, RTL alignment, density spacing, and reactive sender inheritance. ChatMessageMetadata, ChatSystemMessage, and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the checklist remains open.
