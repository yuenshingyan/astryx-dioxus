# ToggleButton standalone checkpoint

`src/toggle_button/component.rs` delegates activation, sizing, elevation, loading, disabled behavior, and tooltips to Button. The caller owns the pressed state. The toggle emits the next state with the original click event; a caller click handler can cancel the request. The component owns `aria-pressed`, swaps the icon, and reserves label width at semibold weight. In forced colors the pressed surface uses Highlight and HighlightText.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `ToggleButton/ToggleButton.tsx` (`42ef476a81a8dfa12bf559e649d09131d17cf5c3`) and its spec (`c0a99d41f63a08a1d83bf32f026568d48b9d9fbe`). The showcase adapts the standalone arrangements with text glyphs and a subset of the color examples. Controlled group selection is covered by the separate [ToggleButtonGroup checkpoint](toggle-button-group-port.md). The grouped Label example and optimistic asynchronous actions remain pending.

All five standard Rust checks passed. SSR verified component-owned pressed semantics and icon replacement. Chrome verified keyboard activation, preserved focus and label width, icon replacement, the accessible pressed state, and forced-colors paint. The showcase navigation regression passed. Published visual parity and manual assistive technology were not verified; the migration checklist remains open.
