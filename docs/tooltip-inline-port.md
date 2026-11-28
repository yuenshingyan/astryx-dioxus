# Tooltip: inline text checkpoint

`src/tooltip/component.rs` adds the plain-text anchor and description path. It reuses the existing Tooltip controller and presentation, attaches a stable description, and does not add a tab stop for empty or redundant help. The showcase adapts the upstream inline-text arrangement.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Tooltip/Tooltip.tsx` (`02158c36794e99c9eb973cd427df8628b72c571e`), the Tooltip spec (`411c1670e4de6ef5c0c30029a98abe593c777489`), and `TooltipInlineTextTooltips.tsx` (`d697a756ba6b62ea23af0e41c4dc65117f4db7f1`). Rich content, caller element and external anchors, configurable presentation and open state, public `useTooltip`, and the remaining catalog arrangements are not implemented.

The inline trigger's hover cue now follows the pinned `Tooltip.tsx` and spec FR8: a dashed underline in `--color-border-emphasized` with a 2px offset. The boolean prop defaults to on for this text-only adapter, and opting out does not remove the tab stop or description. Empty and redundant help still omit both the cue and the tab stop, an existing Rust adapter policy. The shared focus outline and controller are unchanged.

The original checkpoint passed five Rust checks and two Chrome suites, including pointer and Tab reveal, Escape dismissal with retained focus, and showcase navigation. The cue follow-up checks SSR attributes and the emitted stylesheet only; it does not verify raster output or assistive-technology speech. The checklist remains open.
