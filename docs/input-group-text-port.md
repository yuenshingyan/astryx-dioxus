# InputGroupText checkpoint

`src/input_group/input_group_text.rs` ports the noninteractive prefix and suffix cell. It emits the upstream member marker, uses logical corners and a one-border overlap, and isolates plain LTR text in an inner span without changing the cell's direction. Rich nodes and plain text with strong RTL characters retain their direction.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `InputGroup/InputGroupText.tsx` (`eddd28bf0da3f25e2ec3797317e2a0320332cb6c`), its spec (`a46a120d63971ba34f0ddf8fb34a7bb9b13e8ffa`), and the shared group and text-direction helpers. InputGroupText has no separate catalog templates; the showcase adapts the prefix and suffix arrangements from InputGroup Basic and Showcase using native inputs. The [InputGroup checkpoint](input-group-port.md) now composes these cells in a named row; TextInput integration remains pending. The RTL showcase keeps the URL row LTR and separately demonstrates mirrored cell placement with an Arabic price label.

All five standard Rust checks passed. SSR verified the member marker and direction handling. Chrome verified RTL logical corners and border overlap, LTR prefix text, and native input focus. Showcase navigation also passed. Published visual parity and manual assistive technology were not verified; the checklist remains open.
