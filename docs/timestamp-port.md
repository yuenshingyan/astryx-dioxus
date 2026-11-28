# Timestamp: absolute UTC checkpoint

`src/timestamp/` renders whole Unix seconds as a native `<time>` with a UTC `datetime`. The supported display formats are `system_date`, `system_date_time`, `system_time`, and `unix_seconds`. Rendering does not read the clock, locale, or host time zone. Values outside Gregorian years 1–9999 render nothing. This checkpoint requires an explicit format rather than implementing the upstream `auto` default.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Timestamp/Timestamp.tsx` (`968bcb33d21895e24d70d650869d9c0bb38ff895`), its spec (`949b74d90933870d344d65875cf69a9e395f914c`), and the showcase (`ebdfc8f900e5e50934873144f9f6c8ad5d49e91f`). The local showcase keeps the pinned instant and displays the four supported formats.

The original checkpoint passed five Rust checks and three Chrome suites. SSR covered pre-epoch seconds, leap day, year bounds and omission of out-of-range content. Chrome checked all four outputs, native datetime, and the absence of an extra tab stop under emulated time zones.

The default typography follow-up ports the pinned spec's FR1 `supporting` type and `secondary` color through the existing inline `Text` wrapper. The native `<time>` keeps its class, `data-format`, UTC `datetime`, and visible output. Without a hover card, neither element adds an accessible name or a tab stop (FR5–FR6). The props and format variants are unchanged. `cargo test --locked --test timestamp` covers the wrapper defaults, repeated SSR output, and preserved time semantics; browser and visual checks were not rerun.

The checklist remains open. Typography overrides, the explicit `<time>` style reset and wrapper-level Timestamp target, localized and relative formats, time-zone selection, live updates, HoverCard integration, remaining catalog examples, and visual parity are still pending.
