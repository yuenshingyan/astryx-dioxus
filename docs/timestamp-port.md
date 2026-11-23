# Timestamp: absolute UTC checkpoint

`src/timestamp/` renders whole Unix seconds as a native `<time>` with a UTC `datetime`. The supported display formats are `system_date`, `system_date_time`, `system_time`, and `unix_seconds`. Rendering does not read the clock, locale, or host time zone. Values outside Gregorian years 1–9999 render nothing. This checkpoint requires an explicit format rather than implementing the upstream `auto` default.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Timestamp/Timestamp.tsx` (`968bcb33d21895e24d70d650869d9c0bb38ff895`), its spec (`949b74d90933870d344d65875cf69a9e395f914c`), and the showcase (`ebdfc8f900e5e50934873144f9f6c8ad5d49e91f`). The local showcase keeps the pinned instant and displays the four supported formats.

All five Rust checks and three Chrome suites passed. SSR covered pre-epoch seconds, leap day, year bounds and omission of out-of-range content. Chrome checked all four outputs, native datetime, and the absence of an extra tab stop under emulated time zones. Localized and relative formats, time-zone selection, live updates, typography and HoverCard integration, remaining catalog examples, and visual parity remain open.
