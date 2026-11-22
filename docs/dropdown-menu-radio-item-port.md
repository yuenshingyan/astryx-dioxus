# DropdownMenuRadioItem checkpoint

The row composes Item and RadioIndicator inside DropdownMenuRadioGroup. The group owns the checked value. An enabled click requests the value before calling the consumer handler; a disabled row remains discoverable but does not activate. The row does not add a sequential tab stop.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `DropdownMenu/DropdownMenuRadioItem.tsx` (`0820105be070e8ae9c230dfacf54836b8160fa7e`) and its spec (`3e08ae7bf66802fb0921448e6ad4d5c3da0afb6a`). The inline pointer preview adapts the Storybook sorting example. The production menu still needs to supply navigation, hover focus, size context, and close-on-select behavior.

All five standard Rust checks and four Chrome suites passed. SSR and Chrome verified controlled state, disabled behavior, and menuitemradio semantics. Keyboard navigation, touch interaction, published visual parity, and manual assistive technology were not verified. The checklist remains open.
