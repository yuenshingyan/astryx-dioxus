# Tab checkpoint

`src/tab_list/tab.rs` renders an individual tab with caller-owned selection. Navigation mode uses a link or button and `aria-current`; panel mode uses a button with `role="tab"`, `aria-selected`, and a panel reference. Disabled items block activation, and unsafe destinations bypass custom link rendering. The strip owner remains responsible for panel visibility and roving focus.

Sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TabList/Tab.tsx` (`93dda9d33b8da1750c82b48e63655ae111876e5e`), its spec (`5e91ee87324f5a65435d9fc31cc3b8e1f00029cd`), and `TabShowcase.tsx` (`9e64cf572380522f9863c53ef467ec1dbef85aac`). The adapted showcase uses individually tabbable buttons rather than simulating a TabList.

All five Rust checks and three Chrome suites passed. SSR verified panel markup and suppression of the navigation destination in panel mode. Chrome verified caller-controlled selection, Enter activation, focus retention, and the disabled state. [TabList](tab-list-port.md) supplies grouped selection and arrow-key focus. Touch feedback and published visual parity remain open.
