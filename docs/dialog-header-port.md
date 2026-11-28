# DialogHeader checkpoint

`src/dialog/dialog_header.rs` composes LayoutHeader, a native level-two Heading, optional subtitle and action slots, and an optional ghost close button. The title can be referenced by `title_id` and receives mount focus unless `is_inline` is set. The close button requests `false`; the enclosing dialog owns visibility and dismissal. End-slot compensation uses logical edges and defaults to both axes when the close button is present.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Dialog/DialogHeader.tsx` (`71670ee08eb74fb70bad676edde593c923578cf4`), its spec (`083f1ba305c9e53c357c3dc2c16bd23f2f9efdf6`), and both catalog templates. The showcase adapts the templates with inline Layouts and demonstrates explicit title naming in a separate probe. Parent Dialog context, autofocus after `showModal()`, modal focus return, and translation-provider integration remain pending; callers can supply a localized close label.

In the initial port, all five standard Rust checks passed. SSR verified native heading markup, title ID, and inline focus opt-out. Chrome verified inline focus suppression, title focus on mount, the caller-linked dialog name in the accessibility tree, and the close action. DrawerHeader, LayoutHeader, and showcase navigation regressions passed. That probe used a caller-labelled `role="dialog"`, not native modal integration. Published visual parity and manual assistive technology were not verified.

## Follow-up verification

The local TSX and spec match the Git blob hashes above. The pinned source uses
LayoutHeader for the outer inset and divider default; there is no separate
DialogHeader `true` default. The port retains that delegation. The existing end
slot uses fixed `--spacing-2` negative margins on the selected logical edges,
not a second Layout inset. DialogHeader now also uses the source's
`--spacing-0` title/subtitle gap, which is zero in the default tokens.

The close action still delegates to Button and requests `false` without owning
visibility. A whitespace-only `close_label` now falls back to `Close` for the
button and tooltip; explicit nonblank labels are preserved. The new focused
regressions cover SSR name attributes, decorative glyph, non-submit type, inline
subtitle links, trailing slot order and compensation selection. All four focused
SSR tests passed via the serialized Cargo slot after an earlier disk-space
failure. Formatting and bundle verification passed.

SSR attributes are not a live DOM/AX check and do not verify delivery of the
close callback or native pointer and keyboard activation. The logical CSS
declarations were compared by inspection; computed inset and hit geometry
(including RTL) were not measured.
No raster comparison or spoken screen-reader check ran. Parent-owned naming and
focus integration, automatic catalog localization, and the remaining examples
and template comparisons still gate completion. No checklist item was closed.
