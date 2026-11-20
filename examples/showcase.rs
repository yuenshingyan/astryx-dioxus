use std::future::Future;
use std::pin::Pin;

use astryx_dioxus::{Button, ButtonElevation, ButtonSize, ButtonType, ButtonVariant, STYLES};
use dioxus::prelude::*;

#[path = "showcase/aspect_ratio.rs"]
mod aspect_ratio;
#[path = "showcase/badge.rs"]
mod badge;
#[path = "showcase/blockquote.rs"]
mod blockquote;
#[path = "showcase/button_contract.rs"]
mod button_contract;
#[path = "showcase/button_group.rs"]
mod button_group;
#[path = "showcase/button_patterns.rs"]
mod button_patterns;
#[path = "showcase/card.rs"]
mod card;
#[path = "showcase/center.rs"]
mod center;
#[path = "showcase/check_indicator.rs"]
mod check_indicator;
#[path = "showcase/clickable_card.rs"]
mod clickable_card;
#[path = "showcase/divider.rs"]
mod divider;
#[path = "showcase/grid.rs"]
mod grid;
#[path = "showcase/h_stack.rs"]
mod h_stack;
#[path = "showcase/heading.rs"]
mod heading;
#[path = "showcase/icon_button.rs"]
mod icon_button;
#[path = "showcase/kbd.rs"]
mod kbd;
#[path = "showcase/link.rs"]
mod link;
#[path = "showcase/progress_bar.rs"]
mod progress_bar;
#[path = "showcase/section.rs"]
mod section;
#[path = "showcase/selectable_card.rs"]
mod selectable_card;
#[path = "showcase/skeleton.rs"]
mod skeleton;
#[path = "showcase/spinner.rs"]
mod spinner;
#[path = "showcase/stack.rs"]
mod stack;
#[path = "showcase/status_dot.rs"]
mod status_dot;
#[path = "showcase/text.rs"]
mod text;
#[path = "showcase/typography_animation.rs"]
mod typography_animation;
#[path = "showcase/v_stack.rs"]
mod v_stack;
#[path = "showcase/visually_hidden.rs"]
mod visually_hidden;

use aspect_ratio::AspectRatioExamples;
use badge::BadgeExamples;
use blockquote::BlockquoteExamples;
use button_contract::ButtonContractExamples;
use button_group::ButtonGroupExamples;
use button_patterns::DocumentedButtonPatterns;
use card::CardExamples;
use center::CenterExamples;
use check_indicator::CheckIndicatorExamples;
use clickable_card::ClickableCardExamples;
use divider::DividerExamples;
use grid::{GridExamples, GridSpanExamples};
use h_stack::HStackExamples;
use heading::HeadingExamples;
use icon_button::IconButtonExamples;
use kbd::KbdExamples;
use link::LinkExamples;
use progress_bar::ProgressBarExamples;
use section::SectionExamples;
use selectable_card::SelectableCardExamples;
use skeleton::SkeletonExamples;
use spinner::SpinnerExamples;
use stack::{StackExamples, StackItemExamples};
use status_dot::StatusDotExamples;
use text::TextExamples;
use v_stack::VStackExamples;
use visually_hidden::VisuallyHiddenExamples;

const FIGTREE: Asset = asset!("/assets/fonts/Figtree-latin.woff2");

fn main() {
    if let Err(err) = link::register_neutral_icons() {
        eprintln!("Showcase icon setup failed: {err}");
        return;
    }
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    if typography_animation::is_requested() {
        return rsx! { typography_animation::TypographyAnimation {} };
    }
    let mut count = use_signal(|| 0);
    let mut ancestor_count = use_signal(|| 0);
    let mut form_count = use_signal(|| 0);
    let mut is_loading = use_signal(|| false);
    let variants = [
        (ButtonVariant::Primary, "Primary"),
        (ButtonVariant::Secondary, "Secondary"),
        (ButtonVariant::Ghost, "Ghost"),
        (ButtonVariant::Destructive, "Destructive"),
    ];
    let sizes = [
        (ButtonSize::Small, "Small"),
        (ButtonSize::Medium, "Medium"),
        (ButtonSize::Large, "Large"),
    ];

    rsx! {
        document::Title { "Astryx Dioxus Showcase" }
        document::Style { "{STYLES}" }
        document::Style { "@font-face {{font-family:Figtree;src:url('{FIGTREE}') format('woff2');font-style:normal;font-weight:300 900;font-display:swap;}}" }
        document::Style { "
            body {{ font-family: Figtree, system-ui, sans-serif; margin: 0; background: #f8f4ed; }}
            main {{ max-width: 56rem; margin: 3rem auto; padding: 1.5rem; }}
            section {{ margin-block: 2rem; }}
            .showcase-row {{ display: flex; flex-wrap: wrap; align-items: center; gap: 1rem; }}
            .demo-count {{ font-size:12px;line-height:20px;padding:0 6px;border-radius:9999px;background:light-dark(#e2e2e2,#303030);color:light-dark(#1b1b1b,#d4d4d4); }}
            .demo-count--info {{ background:light-dark(#b9d7ff,#88bcff3d);color:light-dark(#003978,#a1caff); }}
            .demo-count--warning {{ background:light-dark(#fae19e,#e2b6233d);color:light-dark(#4b3900,#eec448); }}
        " }
        main { "data-astryx-theme": "astryx",
            h1 { "Astryx Dioxus Showcase" }
            p { "Documented patterns, interaction states, and integrations. See docs/ for each component's verification notes." }
            nav { "aria-label": "Showcase sections",
                a { href: "#button-group-examples", "ButtonGroup examples" }
                " · "
                a { href: "#icon-button-examples", "IconButton examples" }
                " · "
                a { href: "#link-examples", "Link examples" }
                " · "
                a { href: "#spinner-examples", "Spinner examples" }
                " · "
                a { href: "#badge-examples", "Badge examples" }
                " · "
                a { href: "#divider-examples", "Divider examples" }
                " · "
                a { href: "#kbd-examples", "Kbd examples" }
                " · "
                a { href: "#skeleton-examples", "Skeleton examples" }
                " · "
                a { href: "#status-dot-examples", "StatusDot examples" }
                " · "
                a { href: "#progress-bar-examples", "ProgressBar examples" }
                " · "
                a { href: "#blockquote-examples", "Blockquote examples" }
                " · "
                a { href: "#card-examples", "Card examples" }
                " · "
                a { href: "#aspect-ratio-examples", "AspectRatio examples" }
                " · "
                a { href: "#stack-examples", "Stack examples" }
                " · "
                a { href: "#stack-item-examples", "StackItem examples" }
                " · "
                a { href: "#grid-examples", "Grid examples" }
                " · "
                a { href: "#grid-span-examples", "GridSpan examples" }
                " · "
                a { href: "#h-stack-examples", "HStack examples" }
                " · "
                a { href: "#v-stack-examples", "VStack examples" }
                " · "
                a { href: "#visually-hidden-examples", "VisuallyHidden examples" }
                " · "
                a { href: "#center-examples", "Center examples" }
                " · "
                a { href: "#check-indicator-examples", "CheckIndicator examples" }
                " · "
                a { href: "#section-examples", "Section examples" }
            }
            DocumentedButtonPatterns {}
            section { "aria-labelledby": "variants-heading",
                h2 { id: "variants-heading", "Variants" }
                div { class: "showcase-row",
                    {variants.iter().map(|(variant, label)| rsx! {
                        Button { label: *label, variant: *variant }
                    })}
                }
            }
            section { "aria-labelledby": "sizes-heading",
                h2 { id: "sizes-heading", "Sizes" }
                div { class: "showcase-row",
                    {sizes.iter().map(|(size, label)| rsx! {
                        Button { label: *label, size: *size }
                    })}
                }
            }
            section { "aria-labelledby": "states-heading",
                h2 { id: "states-heading", "Interaction and loading" }
                div { class: "showcase-row", onclick: move |_| ancestor_count += 1,
                    Button { id: "counter", label: format!("Clicked {} times", count()), onclick: move |_| count += 1 }
                    Button { id: "disabled", label: "Disabled", is_disabled: true, onclick: move |_| count += 100 }
                    Button { id: "loading", label: "Save", is_loading: true, loading_label: "Loading" }
                    Button { id: "toggle-loading", label: "Toggle loading", onclick: move |_| is_loading.toggle() }
                    Button { id: "controlled-loading", label: "Controlled loading", is_loading: is_loading(), loading_label: "Loading" }
                    Button {
                        id: "async-action",
                        label: "Async action",
                        loading_label: "Loading",
                        click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> {
                            Box::pin(async move {
                                count += 1;
                                let _ = document::eval("await new Promise(resolve => setTimeout(resolve, 800)); return true;").await;
                            })
                        },
                    }
                    Button {
                        id: "interruptible-action",
                        label: "Interruptible action",
                        is_interruptible: true,
                        loading_label: "Loading",
                        click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> {
                            Box::pin(async move {
                                count += 1;
                                let _ = document::eval("await new Promise(resolve => setTimeout(resolve, 800)); return true;").await;
                            })
                        },
                    }
                    Button {
                        id: "prevented-action",
                        label: "Prevented action",
                        onclick: move |event: MouseEvent| event.prevent_default(),
                        click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> {
                            Box::pin(async move { count += 100; })
                        },
                    }
                }
                p { id: "click-count", "Click count: {count}" }
                p { id: "ancestor-count", "Ancestor clicks: {ancestor_count}" }
            }
            section { "aria-labelledby": "content-heading",
                h2 { id: "content-heading", "Content and width" }
                div { class: "showcase-row",
                    Button { label: "Add", icon: rsx! { span { "aria-hidden": "true", "+" } } }
                    Button { id: "icon-only", label: "Add item", is_icon_only: true, icon: rsx! { span { "aria-hidden": "true", "+" } } }
                    Button { label: "Messages", end_content: rsx! { span { "3" } } }
                    Button { label: "Save changes", strong { "Custom content" } }
                    Button { label: "Floating action", elevation: ButtonElevation::High }
                }
                div { style: "width: 140px; margin-top: 1rem;",
                    Button { id: "truncated", label: "A long label that must truncate", width: "100%" }
                }
            }
            section { "aria-labelledby": "links-heading",
                h2 { id: "links-heading", "Links and forms" }
                div { class: "showcase-row",
                    Button { id: "link", label: "Go to variants", href: "#variants-heading" }
                    Button { id: "disabled-link", label: "Disabled link", href: "#variants-heading", is_disabled: true }
                    Button { id: "busy-link", label: "Busy link", href: "#variants-heading", is_loading: true, loading_label: "Loading" }
                }
                form {
                    id: "demo-form",
                    onsubmit: move |event| { event.prevent_default(); form_count += 1; },
                    Button { id: "submit", label: "Submit", button_type: ButtonType::Submit }
                    Button { id: "disabled-submit", label: "Disabled submit", button_type: ButtonType::Submit, is_disabled: true }
                    Button { id: "busy-submit", label: "Busy submit", button_type: ButtonType::Submit, is_loading: true }
                }
                p { id: "form-count", "Submissions: {form_count}" }
            }
            ButtonContractExamples {}
            ButtonGroupExamples {}
            IconButtonExamples {}
            LinkExamples {}
            SpinnerExamples {}
            BadgeExamples {}
            DividerExamples {}
            KbdExamples {}
            SkeletonExamples {}
            StatusDotExamples {}
            TextExamples {}
            HeadingExamples {}
            ProgressBarExamples {}
            BlockquoteExamples {}
            CardExamples {}
            ClickableCardExamples {}
            SelectableCardExamples {}
            AspectRatioExamples {}
            StackExamples {}
            StackItemExamples {}
            GridExamples {}
            GridSpanExamples {}
            HStackExamples {}
            VStackExamples {}
            VisuallyHiddenExamples {}
            CenterExamples {}
            CheckIndicatorExamples {}
            SectionExamples {}
        }
    }
}
