# CommandPaletteInput checkpoint

The standalone input renders a native textbox with a decorative search icon, an accessible name independent of the placeholder, and optional trailing content. It exposes value changes and native input and keyboard events. Autofocus is enabled by default; the showcase opts out so it does not steal focus on page load.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `CommandPaletteInput.tsx` (`8cf14dfc6d48917a13203d3bc6a7a5c69f6ef754`), its spec (`2f6ff1a1231c66d2fc442f0434fd73ba80c9d038`), and `CommandPaletteShowcase.tsx` (`bb61dcb0ccaf25d37295e1d816699ccde0f0b97d`). The preview isolates the query field rather than simulating an opening palette; the trailing keycap is illustrative.

All five Rust checks and three Chrome suites passed. SSR verified the independent name, default autofocus markup, and absence of popup state. Chrome verified editing, accessible textbox naming, the autofocus opt-out, and focus retention. Palette-owned search, busy state, result navigation, and published visual parity remain open.
