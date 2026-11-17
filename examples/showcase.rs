use astryx_dioxus::{Button, ButtonVariant, STYLES};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        document::Title { "Astryx Dioxus" }
        document::Style { "{STYLES}" }
        main { style: "max-width: 56rem; margin: 4rem auto; padding: 1.5rem; font-family: system-ui, sans-serif;",
            h1 { "Astryx Dioxus" }
            p { "A component library migration, one component at a time." }
            section { "aria-labelledby": "buttons-heading",
                h2 { id: "buttons-heading", "Buttons" }
                div { style: "display: flex; flex-wrap: wrap; gap: 1rem;",
                    Button { onclick: move |_| count += 1, "Clicked {count} times" }
                    Button { variant: ButtonVariant::Secondary, "Secondary" }
                    Button { disabled: true, "Disabled" }
                }
                p { role: "status", "Click count: {count}" }
            }
        }
    }
}
