use dioxus::events::onmounted;
use dioxus::prelude::*;

use astryx_dioxus::{
    Card, Grid, GridAlignment, GridColumns, GridSpan, GridSpanColumns, SizeValue, SpacingStep,
    Text, TextColor,
};

#[component]
fn SpanCard(label: &'static str, #[props(default = true)] has_fixed_height: bool) -> Element {
    rsx! {
        Card { height: has_fixed_height.then(|| SizeValue::from(80)),
            Text { color: TextColor::SECONDARY, "{label}" }
        }
    }
}

#[component]
pub(super) fn GridSpanExamples() -> Element {
    let mut mounted = use_signal(|| None);
    let mut mounted_size = use_signal(|| None::<(f64, f64)>);
    rsx! {
        section { id: "grid-span-examples", "data-astryx-theme": "neutral",
            h2 { "GridSpan — original patterns" }
            h3 { "Showcase" }
            Grid { id: "grid-span-showcase", columns: GridColumns::Fixed(4), gap: SpacingStep::Three, width: SizeValue::from(400),
                GridSpan { columns: GridSpanColumns::Count(3), SpanCard { label: "Spans 3 columns" } }
                SpanCard { label: "1 col" }
                GridSpan { rows: 2, SpanCard { label: "1 col", has_fixed_height: false } }
                GridSpan { columns: GridSpanColumns::Count(3), SpanCard { label: "Full-width row" } }
                SpanCard { label: "1 col" }
                GridSpan { columns: GridSpanColumns::Count(2), SpanCard { label: "Spans 2 columns" } }
            }
            h3 { "Columns" }
            Grid { id: "grid-span-columns", columns: GridColumns::Fixed(3), gap: SpacingStep::Three, width: SizeValue::from(400),
                GridSpan { columns: GridSpanColumns::Count(2), SpanCard { label: "Spans 2 columns" } }
                SpanCard { label: "1 col" }
                SpanCard { label: "1 col" }
                GridSpan { columns: GridSpanColumns::Count(2), SpanCard { label: "Spans 2 columns" } }
            }
            h3 { "Full explicit span with an implicit track" }
            Grid { id: "grid-span-explicit", columns: GridColumns::Fixed(2), row_height: 80.0, width: SizeValue::from(240),
                align: GridAlignment::Start, gap: SpacingStep::Three, style: "grid-auto-columns:40px",
                GridSpan { id: "grid-span-implicit", columns: GridSpanColumns::Count(3),
                    "Three tracks"
                }
                GridSpan { id: "grid-span-full", columns: GridSpanColumns::Full,
                    class: "grid-span-caller", style: "grid-column:2;grid-row:2;color:inherit",
                    "data-mounted-size": mounted_size().map(|(width, height)| format!("{width},{height}")),
                    attributes: vec![onmounted(move |event: MountedEvent| {
                        mounted.set(Some(event.data()));
                    })],
                    a { id: "grid-span-long-link", href: "#grid-span-explicit",
                        style: "min-width:0;white-space:nowrap;overflow:hidden;text-overflow:ellipsis",
                        "A long focusable child whose full label remains in the document"
                    }
                    button { id: "grid-span-next-control", onclick: move |_| {
                        if let Some(target) = mounted() {
                            spawn(async move {
                                if let Ok(rect) = target.get_client_rect().await {
                                    mounted_size.set(Some((rect.size.width, rect.size.height)));
                                }
                            });
                        }
                    }, "Next control" }
                }
            }
        }
    }
}
