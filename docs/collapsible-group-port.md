# CollapsibleGroup checkpoint

The nearest group controls named Collapsible items; unnamed disclosures retain standalone state. `CollapsibleGroupValue` pairs the mode with its value: single allows zero or one open item, multiple toggles items independently. A controlled group emits requests without mutating its own selection. Undivided groups add no DOM; divided groups add a passive wrapper, row dividers, and balanced density by default. Panel content resets group presentation without resetting selection scope.

Sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `CollapsibleGroup.tsx` (`37006e4111869cb4fc5139ee547621f01fd1ce00`), `CollapsibleGroupContext.tsx` (`7c9c8c6aae706e15b15fc6c43b6453462eaac78c`), and the spec (`bfe23468910da026056d79aa309c7f0b14f89c4f`). The showcase adapts Single and Multiple Accordion; the single sample uses divided rows to show the wrapper treatment.

All five Rust checks and three Chrome suites passed. SSR verified nearest-group ownership and wrapper omission. Chrome verified keyboard selection, retained trigger focus, independent toggling, row dividers, and wrapperless layout. Group-level chevron defaults, grouped layout animation, remaining catalog arrangements, and published visual parity remain open.
