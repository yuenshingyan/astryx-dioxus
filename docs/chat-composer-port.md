# Chat Composer: plain-text checkpoint

`src/chat/chat_composer.rs` composes the existing ChatComposerInput and ChatSendButton. The caller can own the draft or let the composer keep it locally. Enter and Send submit trimmed nonblank text and request clearing the draft. Stop remains available while editing is disabled. The composer also provides drawer, header, and footer action slots.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Chat/ChatComposer.tsx` (`ac67fbeb71b70ae4d1039705ad2d821dcce89dfb`), its spec (`d96536c26f36578b77a040ecf4d62d7fcc196bfd`), and the showcase (`0eb011e4792727bee6de8915c386b79df53a3df7`). The adapted showcase retains the width and placeholder and uses a simulated running state to exercise Stop. It does not send any messages over a network.

All five Rust checks and the Chat Composer, Chat Composer Input, Chat Send Button, and showcase-navigation Chrome suites passed. The focused suite verified blank-input gating, button and keyboard submission, draft clearing, and Stop while the editor was disabled. Custom input and send-button slots, status announcements, density options, rich entry, and visual parity remain open. The checklist remains open.
