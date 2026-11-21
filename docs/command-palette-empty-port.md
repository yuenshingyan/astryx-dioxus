# CommandPaletteEmpty checkpoint

`src/command_palette/command_palette_empty.rs` centers caller content in one container with supporting typography and secondary text color. It adds no role, live region, or focus stop. Long unbroken strings can wrap, including inside the Astryx Text component. Callers should keep actions outside listbox and option semantics.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CommandPalette/CommandPaletteEmpty.tsx` (`6d2658abe3a9f071bd4392743426a4304bb34433`), the spec (`1ec47de01b41220a0771ebfee94183f8a2406380`), and both catalog templates. The showcase reuses their Text messages without the unported palette and search source. Parent bootstrap/search integration and announcements remain pending.

All five standard Rust checks passed. SSR verified root attributes, caller links, and absence of component-owned semantics. Chrome verified centering, long-token wrapping without horizontal overflow, and both messages in the accessibility tree. Showcase navigation also passed. Published visual parity and manual assistive technology were not verified; the migration checklist remains open.
