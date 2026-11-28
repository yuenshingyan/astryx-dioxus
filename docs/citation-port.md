# Citation checkpoint

`src/citation/component.rs` renders inline source labels and superscript numbers. A valid destination produces a native `doc-noteref` link with `target="_blank"` and `rel="noopener noreferrer"`; a missing or rejected destination produces a named, nonfocusable group. Label artwork is decorative, and a supplied node takes precedence over `src`. The generated English name can be replaced through `accessible_label`.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Citation/Citation.tsx` (`3e244b73857a32e36a2312d2dd3e37b9b165299b`), its spec (`1fc0fc559dfcec7f165eb093a040ac37adf98761`), and the three catalog templates. The showcase uses local lettermarks instead of remote favicons. Rust callers use `src` for image URLs rather than the upstream string form of `icon`. The translation-provider integration remains open.

At the initial checkpoint, all five standard Rust checks passed. SSR verified component-owned roles and names, destination rejection, and decorative artwork. Chrome verified the example count, inline alignment, focusability, new-tab attributes, and names in the accessibility tree. Showcase navigation passed.

The cursor follow-up matches the pinned source for both variants: links use a pointer, links marked `aria-disabled="true"` use the default cursor, and inert references receive neither Citation cursor rule. This is styling only; `aria-disabled` does not block native navigation. The supplied source and spec were compared byte-for-byte with the pinned revision. Focused tests cover the bundled cursor rules and verify that both variants retain safe native link attributes when marked `aria-disabled`.

The migration checklist remains open. Translation-provider integration and FR5 caller-role and ARIA-label passthrough are unchanged. Browser checks were not rerun for this follow-up; published visual comparison, actual new-tab navigation, and manual assistive technology remain unverified.
