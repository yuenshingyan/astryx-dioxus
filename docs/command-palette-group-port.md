# CommandPaletteGroup checkpoint

`src/command_palette/command_palette_group.rs` groups results in a column. The visible heading names the group and is hidden from the accessibility tree to avoid repeating that name. The root and heading have separate styling targets. The caller can override the root role and name, as in the pinned source.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CommandPalette/CommandPaletteGroup.tsx` (`e0cac32fb47e532ca937a9787c1c501549bad9ce`), its spec (`5957d2bf67533e10acce5dc28e90af911c34c845`), and both catalog templates. The showcase keeps their headings and result labels, using native buttons and inline selection feedback in place of the palette, its list and item controls, search, and Toast. Parent integration remains pending.

All five standard Rust checks passed. SSR verified default naming, caller overrides, and root forwarding. Chrome verified group names without duplicate headings in their accessibility subtrees, layout, and native keyboard activation in the adapted examples. CommandPaletteEmpty and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
