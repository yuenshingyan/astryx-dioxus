# CommandPaletteItem standalone checkpoint

`src/command_palette/command_palette_item.rs` renders a caller-controlled option with independent highlight, selection, and disabled states. Enabled items report their value on click after the caller's click handler, unless that handler cancels the event. Disabled items do not activate. The component does not add a tab stop or implement composite keyboard navigation.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CommandPalette/CommandPaletteItem.tsx` (`1f8fb6e8d8ee53b393f595bd8d8c2425e37cdccc`), its spec (`d90a5a937f01f172e01aee70e96ead59c8958cb6`), and both catalog templates. The showcase uses the ported CommandPaletteList, Kbd, and inline feedback instead of the parent palette, search source, and Toast. Parent-owned selection, keyboard navigation, dismissal, highlight scrolling, and the shared press-feedback lifecycle remain pending.

All five standard Rust checks passed. SSR verified option semantics and state reflection. Chrome verified click selection, disabled suppression, and the disabled option in the accessibility tree. CommandPaletteList and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the checklist remains open.
