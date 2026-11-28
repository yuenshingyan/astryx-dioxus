# TabList checkpoint

`src/tab_list/` adds a controlled horizontal strip. Navigation mode renders a named `nav`; panel mode renders a named `tablist` and leaves panel rendering to the caller. The nearest strip owns selection, while the shared list-focus controller keeps a single tab stop. Arrow keys move focus without changing selection; activation requests a new value.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `TabList.tsx` (`8513e9b8f3bbd84d720d535fffc12ccad6f01ac5`) and its spec (`02b0c34070a8dc3a980f162616fb492f387b3e44`). The showcase adds an RTL panel strip with a disabled tab.

The original checkpoint passed five Rust checks and four Chrome suites, including the Toolbar regression suite. That browser run verified the single tab stop, RTL arrow direction, disabled skipping, manual activation, and panel association.

## Horizontal overflow follow-up

The existing strip remains the native scroll container. A private TabList controller uses the existing shared resize observer and measures which physical edges have more content in LTR and RTL. CSS masks the clipped edge or edges without fading the divider rail. Forced colors bypass the masks, preserving the focus outline and the selected tab's `Highlight` indicator. The existing roving-focus controller is unchanged: the overflow controller only scrolls, revealing the focused tab first or the selected tab when focus is outside. It drops the masks while a focused tab is too wide to clear them. The native scrollbar is retained.

The showcase has a narrow, divided navigation strip. `tests/tab_list.rs` checks SSR semantics and the single tab stop; `tests/tab_list.browser.mjs` adds cases for LTR/RTL fades, focus and selection reveal, resizing to fit, and forced colors. Those browser cases were not run for this follow-up, so actual paint (including the divider and focus ring) and cross-browser behavior are unverified.

The checklist stays open. This slice does not add the public overflow modes, hover-pointer scroll arrows, smooth-scroll or mask transitions, keyboard hints, fill, or full bleed / `edge="bleed"` geometry. Published visual parity remains open.
