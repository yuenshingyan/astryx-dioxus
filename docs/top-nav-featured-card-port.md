# TopNavMegaMenuFeaturedCard checkpoint

`src/top_nav/top_nav_mega_menu_featured_card.rs` renders the optional image, title, description, CTA, and caller content in order. Images without meaningful alt text are decorative. The CTA requires both a label and destination, uses the LinkProvider renderer when present, and mirrors its decorative arrow in RTL.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `TopNav/TopNavMegaMenuFeaturedCard.tsx` (`c180f7d1a2e9cd24c4beefc3b097e1f04a8dae8b`), its spec (`53740cfc351cef5c3af3b976b30ae4f54aa352b1`), and both catalog templates. Both text-only arrangements render standalone, as in the pinned templates. The supplemental RTL example reuses an existing offline image. The parent panel's input-specific dismissal and focus return remain pending.

All five standard Rust checks passed. SSR verified image semantics and conditional CTA rendering. Chrome verified offline image geometry, RTL arrow order and transform, accessible link names, and native navigation. TopNavHeading and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the checklist remains open.
