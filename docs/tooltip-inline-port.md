# Tooltip: inline text checkpoint

`src/tooltip/component.rs` adds the plain-text anchor and description path. It reuses the existing Tooltip controller and presentation, attaches a stable description, and does not add a tab stop for empty or redundant help. The showcase adapts the upstream inline-text arrangement.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Tooltip/Tooltip.tsx` (`02158c36794e99c9eb973cd427df8628b72c571e`), the Tooltip spec (`411c1670e4de6ef5c0c30029a98abe593c777489`), and `TooltipInlineTextTooltips.tsx` (`d697a756ba6b62ea23af0e41c4dc65117f4db7f1`). Rich content, caller element and external anchors, configurable presentation and open state, public `useTooltip`, and the remaining catalog arrangements are not implemented.

All five Rust checks and two Chrome suites passed. SSR verified the description relationship and tab-stop policy; Chrome verified pointer and Tab reveal, Escape dismissal with retained focus, and showcase navigation. Published visual parity and manual assistive technology were not verified. The checklist remains open.
