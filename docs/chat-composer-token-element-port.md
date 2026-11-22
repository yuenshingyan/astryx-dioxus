# ChatComposerTokenElement checkpoint

The inline wrapper delegates structured tokens to Badge and renders custom tokens through their supplied callback. It preserves the exact serialized value, keeps the wrapper noneditable, and adds no role or tab stop. Caller attributes reach the root without overriding the serialization markers or editability.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: the helper in `Chat/ChatComposerInput.tsx` (`3c26d5b0708ebd135a2f9b557bd2eebd6391b80f`), its spec (`bd290a31e4fcb5c7edf369356cf3d2ddd2cf783c`), and the BadgeToken and CustomToken stories (`157c2a724598ae5c4e75011c0f9d55f97920e475`). The preview puts both tokens in a native editable host, not a production composer.

All five Rust checks and three Chrome suites passed. SSR verified serialization, attribute precedence, and Badge styling passthrough. Chrome verified that both wrapper types remain noneditable in an editable host. Production composer insertion and deletion, caret behavior, and published visual parity remain open.
