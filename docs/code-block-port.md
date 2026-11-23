# Code Block: plain-source checkpoint

`src/code_block/` renders escaped source in native `<pre><code>` markup. It supports a title, optional line numbers with a source offset, line emphasis, wrapping, a bounded scroll region, and a copy action with caller-supplied labels and status messages. Copy passes the original string to the Clipboard API, not the rendered gutter or selection. The card uses the scoped SyntaxTheme background when available; source is not tokenized.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `CodeBlock/CodeBlock.tsx` (`a1c73288e5eae6687ac9019782c9c1a36f20b7d1`), its spec (`abcd7f4e002496acc13768bea53074d9fdf77f81`), and the `useUser.ts` showcase (`3b03bbe9504521f77e63e1cb7fae47a023920e87`). The local showcase retains the pinned source, title, gutter, and copy action, without syntax highlighting.

All five Rust checks and three Chrome suites passed. The Code Block suite checked the keyboard tab stop, hidden gutter semantics, the exact string passed to a mocked clipboard, and the success and failure status text. Native clipboard permission behavior, spoken status, language grammars, diff, collapsible sections, headerless copy placement, remaining catalog examples and visual parity remain open. The checklist remains open.
