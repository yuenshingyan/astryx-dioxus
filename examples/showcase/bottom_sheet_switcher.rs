use dioxus::prelude::*;

use astryx_dioxus::{
    BottomSheet, BottomSheetHeight, BottomSheetSwitcher, Button, ButtonVariant, DialogPurpose,
};

#[component]
pub(super) fn BottomSheetSwitcherExamples() -> Element {
    let mut active = use_signal(|| None::<String>);
    let mut saved = use_signal(|| false);
    rsx! {
        section { id: "bottom-sheet-switcher-examples",
            h2 { "Bottom Sheet Switcher" }
            Button { label: "Review settings", onclick: move |_| { saved.set(false); active.set(Some("review".into())); } }
            output { if saved() { "Settings saved for this demo" } }
            BottomSheetSwitcher { active_sheet: active(), on_active_sheet_change: move |next| active.set(next),
                BottomSheet { sheet_id: "review", label: "Review notification settings", height: BottomSheetHeight::Hug, purpose: DialogPurpose::Form,
                    h3 { "Review settings" }
                    p { "Daily summaries will be sent by email." }
                    Button { label: "Cancel", variant: ButtonVariant::Secondary, onclick: move |_| active.set(None) }
                    Button { label: "Continue", onclick: move |_| active.set(Some("confirm".into())) }
                }
                BottomSheet { sheet_id: "confirm", label: "Confirm settings", height: BottomSheetHeight::Hug,
                    h3 { "Confirm settings" }
                    p { "Your notification settings are ready to save." }
                    Button { label: "Back", variant: ButtonVariant::Secondary, onclick: move |_| active.set(Some("review".into())) }
                    Button { label: "Save", onclick: move |_| { saved.set(true); active.set(None); } }
                }
            }
        }
    }
}
