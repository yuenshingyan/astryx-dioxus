# Chat Composer Input: plain-text checkpoint

`src/chat/chat_composer_input.rs` provides a named native textarea with local or caller-owned text. Enter submits trimmed nonblank text when a submit callback is present, Shift+Enter inserts a newline, and composing Enter does not submit. Successful submission emits an empty draft without moving focus. The browser caps textarea growth where CSS `field-sizing` is supported. The standalone demo echoes text locally; it does not send messages over a network.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Chat/ChatComposerInput.tsx` (`3c26d5b0708ebd135a2f9b557bd2eebd6391b80f`), its spec (`d816d5a5334ca08b0f56843140cbf4539b018b2b`), and showcase (`7fca34d053883be56ce4b367b13924757df76850`). The local showcase retains the placeholder and width but substitutes a textarea for upstream rich editing.

All five Rust checks and the Chat Composer Input, Chat Composer Drawer, and showcase-navigation Chrome suites passed. The focused suite checked the accessible textbox name, blank submission guard, Shift+Enter, a synthetic composing key, trimmed submission, clearing, and retained focus. Native IME testing, token editing and trigger menus, history, file paste, parent composer integration, and visual parity remain open. The checklist remains open.
