# ChatSendButton checkpoint

`src/chat/chat_send_button.rs` composes Button for standalone send and stop states. Stop remains actionable regardless of send readiness; the selected action precedes the caller's click handler. Button owns focus and suppresses activation for the focusable `aria-disabled` send state. Callers own state transitions and can supply localized labels and replacement icons.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatSendButton.tsx` (`5eb063911b7fb17c654ed3e7f12c2604da2053c8`), its spec (`aeff7e9cd4341b0a78f43686eea8ff04b3be517e`), and four catalog templates. The composer example uses a native input, and examples and built-in fallbacks use local glyphs. ChatComposer context, shared icon registry and translation integration remain pending.

All five standard Rust checks passed. Chrome verified send/stop transitions, retained focus, action order, disabled activation suppression, and the stop button's accessible name. ChatSystemMessage and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The checklist remains open.
