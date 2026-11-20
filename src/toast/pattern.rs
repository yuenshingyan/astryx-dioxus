// Private published-demo slice of the pinned Toast owner. Not a public Toast API.
// See docs/stack-toast-port.md and THIRD_PARTY_NOTICES.md.
use astryx_dioxus::{Button, ButtonSize, ButtonVariant, Icon, IconSize};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct DemoToast {
    id: u64,
    body: String,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct DemoToasts {
    entries: Signal<Vec<DemoToast>>,
    sequence: Signal<u64>,
}

pub(crate) fn use_demo_toasts() -> DemoToasts {
    DemoToasts {
        entries: use_signal(Vec::new),
        sequence: use_signal(|| 0),
    }
}

impl DemoToasts {
    pub(crate) fn show(mut self, body: &str) {
        self.sequence += 1;
        let id = *self.sequence.peek();
        self.entries.write().push(DemoToast {
            id,
            body: body.into(),
        });
    }
}

#[component]
pub(crate) fn DemoToastViewport(
    toasts: DemoToasts,
    label: String,
    dismiss_label: String,
) -> Element {
    let id = use_hook(|| format!("stack-toast-{}", dioxus::core::current_scope_id().0));
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send("dispose");
        }
    });
    let entries = toasts.entries.read();
    // Upstream's default cap hides oldest entries, rather than dropping them.
    let visible = entries.iter().skip(entries.len().saturating_sub(5));
    rsx! {
        div {
            id: id.clone(), class: "astryx-demo-toast-viewport", popover: "manual",
            role: if entries.is_empty() { None } else { Some("region") },
            "aria-label": label, tabindex: "-1",
            onmounted: move |_| {
                let eval = document::eval(concat!(
                    include_str!("../hooks/use_announce/use_announce.js"),
                    "\n", include_str!("pattern_viewport.js")
                ));
                let _ = eval.send(id.clone());
                controller.set(Some(eval));
            },
            for entry in visible {
                DemoToastRow { key: "{entry.id}", entry: entry.clone(), toasts, dismiss_label: dismiss_label.clone() }
            }
        }
    }
}

#[component]
fn DemoToastRow(entry: DemoToast, toasts: DemoToasts, dismiss_label: String) -> Element {
    let id = format!("stack-toast-row-{}", entry.id);
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send("dispose");
        }
    });
    rsx! {
        div { id: id.clone(), class: "astryx-demo-toast-row", "data-toast-id": "{entry.id}",
            onmounted: move |_| {
                let mut eval = document::eval(concat!(
                    include_str!("../theme/pattern_media.js"),
                    "\n", include_str!("pattern_toast.js")
                ));
                let _ = eval.send(id.clone());
                controller.set(Some(eval));
                spawn(async move {
                    if let Ok(message) = eval.recv::<String>().await
                        && message == "remove"
                    {
                        toasts.entries.write().retain(|toast| toast.id != entry.id);
                    }
                });
            },
            div { class: "astryx-demo-toast-clip",
                div { class: "astryx-toast", "data-type": "info",
                    div { class: "astryx-demo-toast-media",
                        div { class: "astryx-demo-toast-inner",
                            div { class: "astryx-demo-toast-body", "{entry.body}" }
                            div { class: "astryx-demo-toast-end",
                                Button {
                                    class: "astryx-toast-close", label: dismiss_label,
                                    variant: ButtonVariant::Ghost, size: ButtonSize::Small,
                                    is_icon_only: true,
                                    icon: rsx! { Icon { size: IconSize::Small, icon: rsx! {
                                        svg { width: "1em", height: "1em", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", "aria-hidden": "true",
                                            path { d: "M18 6 6 18" } path { d: "m6 6 12 12" }
                                        }
                                    } } },
                                    onclick: move |_| {
                                        if let Some(eval) = *controller.peek() {
                                            let _ = eval.send("dismiss");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
