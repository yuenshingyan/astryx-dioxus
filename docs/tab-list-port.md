# TabList checkpoint

`src/tab_list/` adds a controlled horizontal strip. Navigation mode renders a named `nav`; panel mode renders a named `tablist` and leaves panel rendering to the caller. The nearest strip owns selection, while the shared list-focus controller keeps a single tab stop. Arrow keys move focus without changing selection; activation requests a new value.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `TabList.tsx` (`8513e9b8f3bbd84d720d535fffc12ccad6f01ac5`) and its spec (`02b0c34070a8dc3a980f162616fb492f387b3e44`). The showcase adds an RTL panel strip with a disabled tab.

All five Rust checks and four Chrome suites passed, including the Toolbar regression suite. Chrome verified the single tab stop, RTL arrow direction, disabled skipping, manual activation, and panel association. Overflow fades, fill and full-bleed layout, keyboard hints, and published visual parity remain open.
