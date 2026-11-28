# Bottom Sheet: standalone modal checkpoint

`src/bottom_sheet/` composes the native Dialog controller with a viewport-bottom surface. The caller owns open state, content, and explicit actions. Dialog purpose governs Escape and backdrop dismissal. This checkpoint supports Hug, Capped, Tall, and custom height budgets.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `BottomSheet/BottomSheet.tsx` (`425781873e94176ba2bad67b923d7296479a54fc`), `BottomSheetPanel.tsx` (`621084a7160536078dfb5704654f96a957c236f0`), the spec (`8fda0357a9ce0672f206fd78d9d262c55c8d7f61`), and the Filters showcase (`88de789276a88b6601ed56da66a8aa952eb0c995`). The adapted showcase keeps the three filter choices and caller-owned Apply action.

At the original checkpoint, five Rust checks and five Chrome suites passed. The focused browser suite verified the accessible modal name, bottom placement and default height cap, initial content focus, Escape and Apply dismissal, focus return, and retained filter selection.

### Initially open standalone sheets (FR17)

An initially open standalone sheet now renders the same closed markup as an initially closed sheet. After its content mounts on the client, BottomSheet forwards the caller's current open state to the existing native Dialog controller. The server and first client render do not claim modality before the native host can open. Switcher-managed steps are unchanged.

`tests/bottom_sheet.rs` compares server markup for initially open and closed sheets. Browser hydration, forwarding the request on mount, and native presentation were not reverified for this follow-up. The settled-versus-animated entry distinction in FR17 is still open, along with nonmodal presentation, swipe and snap points, keyboard-aware expansion, scroll locking, transitions, and visual parity. The checklist remains open.
