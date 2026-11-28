# FormLayout checkpoint

`src/form_layout/` renders a plain `div` and owns only outer arrangement. Vertical uses a flex column, horizontal uses implicit `1fr` grid columns in logical order, and horizontal labels use an `auto 1fr` grid that becomes a flex column at viewport widths of 480px or less. The spacing tokens and stylesheet cascade are unchanged. A caller supplies any native form, children, validation, and submission.

The source of record is Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `FormLayout.tsx` (`aab97546c685409a1faa9048d5205fa868d723fa`), `FormLayoutContext.ts` (`fad5cf8885dc29e636a94b37e3e330f3d7d1e2b9`), and the FormLayout component spec. The five pinned templates now use the existing Rust TextInput, TextArea, Selector, CheckboxList, and CheckboxListItem. Selector is native and CheckboxList selection lives in the example's caller-owned state rather than on the list itself. The pinned Showcase and Nested templates intentionally have the same structure and starter values.

## Verification boundaries

- The new regression tests exercise live provider updates, nearest-provider replacement including omitted settings, all three settings resetting at `LayerContentBoundary`, and a fresh provider inside the boundary. Unset entry behavior is published as `None`; the compatible input, not the layout, must resolve that to `Complete`.
- The new SSR tests assert the nonsemantic root, owned direction reflection and caller attributes, native form ownership, optionality markers and inherited `aria-required` on existing text controls, the binary-control exemption, and all five source-derived arrangements. The focused Rust run stopped during compilation with `No space left on device`; the new tests are not verification evidence until they pass. SSR does not prove browser layout or spoken output.
- The earlier Chrome pass on native stand-ins verified computed spacing, equal horizontal tracks in LTR/RTL, the inclusive breakpoint, and native label association. The revised test below exercises actual Field composition for wide RTL and narrow settings, computed geometry, DOM label activation, and Chrome AX roles/names. It needs an integrator run on the new showcase. Neither pass provides paint or raster parity with the published examples. Spoken screen-reader output, touch, and other engines are not verified.

```sh
SHOWCASE_URL=http://127.0.0.1:8145 node tests/form_layout.browser.mjs
```

## Remaining owner dependencies

- DateInput, DateTimeInput, TimeInput, and NumberInput do not consume the default entry policy or expose the field override. Commit timing, error clearing, invalid state, announcements, and caller-status independence must be verified in those controls after integration.
- `LayerContentBoundary` resets all FormLayout settings, but `src/dialog/component.rs` does not wrap its children in that boundary. Automatic Dialog isolation needs the Dialog/Layer owners; a FormLayout cannot reset context for an arbitrary native `dialog` element.
- The required-default check is not complete across the input family. For example, Selector does not read `FormLayoutContext`, so an unmarked selector does not expose inherited `aria-required`. FormLayout must not set native `required` on descendants itself.
- The spec's open question about a tighter gap within each collapsed label/control pair is a design decision. The pinned gap remains unchanged.
