# Overflow List: single-row checkpoint

`src/overflow_list/` measures its own row and collapses items from a logical edge as space changes. It supports a minimum and maximum visible count and a caller-rendered indicator that receives each hidden item and its original index. The private `src/hooks/use_overflow.*` controller owns resize observation and cleanup. An inert measurement row renders copies of the items and the indicator; avoid fixed DOM IDs or mount-side effects in that content. The indicator is measured with all items, as in the upstream implementation.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `OverflowList/OverflowList.tsx` (`e49086ea8810a01933efaf1ffbf120ee06db7c43`), `hooks/useOverflow.ts` (`b4434b0a454e3bf392ddba24bb45d1879936c129`), the spec (`c727b0dbaa503c87d3268eaa351d6faf4fb148c2`), and the showcase (`14aa1bc055dcff4d265fb865e680626e0ba3b2a6`). The showcase retains the five buttons and count indicator; the indicator does not open a menu.

After integrating the earlier local checkpoint with the split stylesheets, the CSS bundle check, all five Rust checks, and the Overflow List, Carousel, and showcase-navigation Chrome suites passed. Chrome measured the indicator fitting after shrinking, verified the visible prefix and hidden count, checked that only one indicator was exposed to accessibility, and verified that all items reappeared when the row widened. Those results predate the notification change below.

## Single-row notifications

`OverflowListWithNotifications` adds an `on_overflow_change` callback independent of the optional indicator. Supply the existing list options as `list: OverflowListProps`. This keeps the original component, props fields, and props builder source-compatible. An exact upstream-style callback prop on `OverflowList` would require approval to change the existing public Rust props shape.

The private overflow controller still measures, observes, and cleans up the same single row. The component waits for the observer to acknowledge its current keyed inputs and count before reporting. A starting fit is silent; collapse reports the hidden items in original index order; grow-back reports an empty vector. Reports compare original indices and root VNode keys, not element content or callback identity. Supply explicit root keys for dynamic items when same-index replacement or reordering must be distinguished. Keyed reports do not make the index-keyed row wrappers preserve mounted item identity across reorders.

The showcase displays the notification count and collapsed labels outside the row and can swap two actions. The Rust unit test covers report deduplication and stale-revision rejection. `tests/overflow_list.browser.mjs` covers fit silence, collapse payload, unchanged resize, same-count keyed reorder, and grow-back alongside the existing indicator checks; the updated browser suite is unrun.

Still open: multi-row wrapping, parent observation, focus on collapse, stable mounted identity across reorders, and visual parity. The existing pre-measure render still applies the cap. Upstream FR7/PR1–PR2 first-paint timing, batched cross-list measurement, and the observer-loop deferral policy are not ported or claimed by this Dioxus post-render callback. The checklist remains open.
