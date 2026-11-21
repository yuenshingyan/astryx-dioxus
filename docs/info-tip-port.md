# InfoTip checkpoint

`src/info_tip/` provides a named native button for short, noninteractive text. The shared Tooltip controller owns presentation and dismissal; InfoTip owns the preview and pin semantics. The button references its description while closed, sets `aria-controls` when shown, and sets `aria-expanded` only while pinned.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `InfoTip/InfoTip.tsx` (`7b16c30fa8012c8b0d26b287c8217b574dcbd8ff`), its spec (`d4a0543a3fb65220ede1ae8e7f50f2a14c7b665f`), and three adapted catalog arrangements. The port uses a local glyph and plain-text help. FieldLabel integration, rich content, registry and translation integration, and expanded touch hit areas remain open.

All five standard Rust checks and three Chrome suites passed. SSR verified the closed description relationship. Chrome verified focus preview, Enter pinning, second-activation dismissal, and Escape with retained focus. Touch, spoken announcements, and published visual parity were not verified. The migration checklist remains open.
