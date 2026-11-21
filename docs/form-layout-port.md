# FormLayout checkpoint

`src/form_layout/` implements the vertical, horizontal, and horizontal-label layouts on a native `div`. Horizontal labels collapse at viewport widths of 480px or less. The private context publishes direction, optionality, and entry-error defaults. A nested FormLayout replaces these settings, and a layer boundary resets them. FormLayout does not submit forms, set native validation attributes, or own the controls.

The implementation follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `FormLayout.tsx` (`aab97546c685409a1faa9048d5205fa868d723fa`), `FormLayoutContext.ts` (`fad5cf8885dc29e636a94b37e3e330f3d7d1e2b9`), the component spec, and five catalog templates. The examples substitute native controls for the unported Field, TextInput, TextArea, Selector, and CheckboxList. Actual control integration for optionality and entry-error defaults remains pending.

All five standard Rust checks passed. SSR verified nested context shadowing and layer resets. Chrome verified the inclusive 480px breakpoint, logical RTL order, equal horizontal tracks, spacing, and native label association. FieldStatus and showcase navigation regressions passed. Published screenshot parity, other browsers, and manual assistive technology remain unverified; the migration checklist stays open.

```sh
SHOWCASE_URL=http://127.0.0.1:8145 node tests/form_layout.browser.mjs
```
