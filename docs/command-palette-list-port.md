# CommandPaletteList standalone checkpoint

`src/command_palette/command_palette_list.rs` renders a named, scrollable listbox for caller-owned options and groups. It defaults to “Commands” and forwards the root ID, styles, and native attributes. Selection and keyboard navigation are not owned by this component.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CommandPalette/CommandPaletteList.tsx` (`eb335986a9f57c542d81620fbedddf0363186219`), its spec (`c08262434927b1a3304bac633688e222490c8c4c`), and both catalog templates. The showcase preserves their result arrangements as static previews, using the ported CommandPaletteGroup but not simulating CommandPaletteItem, selection actions, or Toast. Parent-owned result IDs, input linkage, and suppression of listbox semantics for empty or pending results remain open.

All five standard Rust checks passed. SSR verified default and explicit naming, role ownership, and attribute forwarding. Chrome verified listbox names, grouped and disabled options in the accessibility tree, and vertical scrolling. CommandPaletteGroup, CommandPaletteFooter, and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the checklist remains open.
