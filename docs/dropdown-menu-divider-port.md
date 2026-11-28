# DropdownMenuDivider checkpoint

## Pinned source and showcase

The upstream pin is Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`.
The supplied `DropdownMenuDivider.tsx` (`b8dbdeedc3a156752423193ba08caf432146a3a8`)
and spec (`b5e6c07884a1319ef08ba0d358a5472cbd26bbcd`) match the pinned Git tree.
Additional files were fetched at the pin and checked against their tree blobs:

- `DropdownMenuDivider.doc.mjs` (`313f91d3c591d56fd2cf605ae417f1863c539397`): no variants or own named inline/catalog examples. The tree has no divider-specific catalog block.
- `DropdownMenuDivider.stories.tsx` (`100c65229716a70d7d1cb8952693796ca268fb6f`): **MenuSeparator**, with an open compound pointer menu, Edit, a separator, and destructive Delete. The showcase now uses actual menu rows for this story instead of static labels.
- `DropdownMenuGroupShowcase.tsx` and `DropdownMenuGroupWithSelectable.tsx` use compound dividers. `DropdownMenuActions.tsx`, `DropdownMenuNoChevron.tsx`, and `DropdownMenuWithDisabledItems.tsx` use data dividers. These are family examples, not additional divider variants. Pinned `renderDropdownItems.tsx` (`c56f3f36c3fca5f71da82a065c35a1a1a8e861fb`) renders the same divider as compound mode; DropdownMenu, ContextMenu, and BreadcrumbItem call that shared renderer.
- `Divider.tsx` (`0a16240711df301852d2085e21e3a6fd924826b5`), `BaseProps.ts`, `utils/themeProps.ts`, and `theme/tokens.stylex.ts`: default horizontal, unlabeled, full-width subtle rule (`--border-width`, `--color-border`) with `--spacing-1` (`0.25rem`) on both block edges. The Rust port delegates paint and geometry to Divider and keeps both theme classes on its visible root.

## Detailed checklist audit

- **Single horizontal separator — closed by source comparison.** The Rust component exposes no size, color, orientation, or label variants and delegates to the same Divider defaults. The compound showcase mirrors the pinned Storybook story. This is not a rendered comparison claim.
- **Separator role, arrow traversal, and data parity — open.** The separator owns `role="separator"` and horizontal orientation and filters tab/focus overrides. DropdownMenu's controller collects only menu-item roles, but actual Up/Down traversal across the separator in the new demo has not been run. The Rust DropdownMenu, ContextMenu, and BreadcrumbItem APIs do not expose their upstream pointer-menu data path; parity requires those owners to route data divider entries through this component and test against the compound rendering. Touch data dividers are owned by the parent menus.
- **Host inputs and styling — open pending validation.** Supported root ID/data/ARIA attributes and native handlers pass to Divider. The new regression guards against case-insensitive HTML overrides via spread attributes. Rust `onmounted` is the root-ref adapter; `class` and inline `style` are the existing CSS inputs for upstream `xstyle` spacing/layout customization, not a StyleX prop or a visual variant. The tests cover caller inputs and listener registration; the computed spacing override and native delivery on the actual DOM root have not been verified for this revision.

## Remaining verification

Formatting, the CSS bundle check, and browser script syntax passed. The single `cargo-slot test --locked --test dropdown_menu_divider` invocation reached compilation of `astryx-dioxus` but hit the command timeout before a test result. The full Rust suite and showcase build were not run.

The earlier checkpoint recorded Chrome separator AX, block-spacing, and geometry results on the static fixture. It did not compare upstream screenshots or exercise traversal in a menu. On the integrator's preview, run `SHOWCASE_URL=<preview-url> node tests/dropdown_menu_divider.browser.mjs` with the shared CDP lock. The expanded test checks traversal in both directions, rejection of programmatic focus, AX role/name, and default geometry. Compare the open state to the pinned Storybook story across neutral/butter light/dark, RTL, and a narrow viewport (as in pinned `DropdownMenuDividerAudit.a11y.chromium.spec.ts`); manually verify the separator is not a focus stop and that adjacent actions remain reachable with assistive technology. Neither browser nor preview was started here.
