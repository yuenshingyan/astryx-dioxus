# TextInput checkpoint

`src/text_input/` composes Field with a controlled native input. It supports text, password, email, search, telephone, and URL types, a decorative start icon, inherited size, and linked descriptions and status messages. Disabled inputs stay focusable and are omitted from form data; read-only inputs still submit.

Sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TextInput/TextInput.tsx` (`1238f45e2d9550e7fee049812637dba586f33665`) and its spec (`81265924d60251b6add6c4a2befbb5b3fa63d4ed`). The showcase adapts parts of Showcase, Types, and States. The opt-in [clear action](input-clear-button-port.md) is available. InputGroup integration, async state, disabled-reason help, and the remaining catalog arrangements stay open.

All five Rust checks and four Chrome suites passed. SSR verified the hidden label association, native search type, and disabled form semantics. Chrome verified editing, focus retention, description and validation associations, and disabled versus read-only form submission. Published visual parity and spoken output were not verified; the checklist remains open.
