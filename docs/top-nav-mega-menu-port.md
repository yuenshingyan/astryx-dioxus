# Top Nav Mega Menu: desktop disclosure checkpoint

`src/top_nav/top_nav_mega_menu.rs` composes the existing Top Nav item and featured-card slots in a desktop popover. The button exposes expanded state and controls a named group of navigation links, not an ARIA action menu. The shared Popover controller handles Escape, outside dismissal, and focus return when closing from inside the panel. The adapted Products showcase keeps the four links and featured card, uses local fragment targets, and omits decorative catalog icons.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: core `TopNav/TopNavMegaMenu.tsx` (`351d9a8567e0edd617c1e770a382ea8caaec1397`), its spec (`367bbcd9c3474efa16752e3513d35aaca7f1e1af`), and the showcase (`d6f6acfc3e0cac901f78eb7079a1e3739c957907`).

All five Rust checks and four Chrome suites passed. The focused suite verified keyboard opening, accessible group and link names, Tab traversal, Escape focus return, and outside dismissal. Hover timing, close-on-link activation, mobile drawer integration, viewport alignment parity, transitions, and visual parity remain open. The checklist remains open.
