# ChatSendButton checkpoint

`src/chat/chat_send_button.rs` composes Button for standalone send and stop states. Stop remains actionable regardless of send readiness; the selected action precedes the caller's click handler. Button owns focus and suppresses activation for the focusable `aria-disabled` send state. Callers own state transitions and can supply localized labels and replacement icons.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatSendButton.tsx` (`5eb063911b7fb17c654ed3e7f12c2604da2053c8`), its spec (`aeff7e9cd4341b0a78f43686eea8ff04b3be517e`), and four catalog templates. The composer example uses a native input, and examples and built-in fallbacks use local glyphs. ChatComposer context and shared icon registry integration remain pending.

## Accessible-name follow-up

The default name now resolves through `InternationalizationProvider` using the pinned `@astryx.chatSendButton.send` and `@astryx.chatSendButton.stop` keys (FR1, FR2, AR2). Nonblank `send_label` and `stop_label` values still override their respective translations. Blank overrides defer to the catalog; missing or blank catalog entries fall back to English. Button still owns rendering the native control and its `aria-label`.

Focused SSR coverage checks the state-specific translated names, explicit label precedence, and fallback behavior. The original checkpoint passed all five standard Rust checks, and Chrome verified send/stop transitions, retained focus, action order, disabled activation suppression, and the stop button's accessible name. ChatSystemMessage and showcase navigation regressions also passed at that checkpoint. This follow-up did not rerun browser checks. Published visual parity and manual assistive technology remain unverified. The checklist remains open.
