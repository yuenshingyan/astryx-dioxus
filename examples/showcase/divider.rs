use astryx_dioxus::{
    Button, Divider, DividerLabel, DividerOrientation, DividerVariant, Text, TextType,
};
use dioxus::events::onclick;
use dioxus::prelude::*;

// Native example framing matches the source Card/Section geometry. These are
// not reusable Card/Section ports or claims that their checklists are complete.
const SURFACE: &str = "box-sizing:border-box;min-width:0;max-width:100%;overflow:clip;border:var(--border-width) solid var(--color-border);border-radius:var(--radius-container);background:var(--color-background-card);padding:calc(var(--spacing-3) - var(--border-width));--container-padding-inline-start:var(--spacing-3);--container-padding-inline-end:var(--spacing-3);--container-padding-block-start:var(--spacing-3);--container-padding-block-end:var(--spacing-3)";
const SECTION: &str =
    "box-sizing:border-box;width:fit-content;max-width:100%;padding:var(--spacing-3)";
const COLUMN: &str = "display:flex;flex-direction:column;gap:var(--spacing-3)";

#[component]
fn ExampleSurface(children: Element, #[props(default)] is_intrinsic: bool) -> Element {
    let width = if is_intrinsic {
        "width:max-content"
    } else {
        "width:400px"
    };
    rsx! { div { style:"width:fit-content;max-width:100%", "data-divider-reference-frame":"section-outer",
        div { class:"demo-divider-section",style:SECTION,"data-divider-reference-frame":"section",
            article { class:"demo-divider-card",style:"{SURFACE};{width}","data-divider-reference-frame":"card", {children} }
        }
    } }
}

#[component]
pub fn DividerExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut is_labelled = use_signal(|| true);
    let mut events = use_signal(|| 0);
    let custom = DividerVariant::new("custom-rule")?;
    rsx! {
        section {
            id:"divider-examples","data-astryx-theme":"neutral","data-theme":if is_dark(){"dark"}else{"light"},
            style:if is_dark(){"color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem;font-size:var(--text-body-size);line-height:var(--text-body-leading)"}else{"color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem;font-size:var(--text-body-size);line-height:var(--text-body-leading)"},
            h2 { "Divider — original examples" }
            Button { id:"dv-theme",label:"Switch Divider light/dark",onclick:move |_|is_dark.toggle() }
            h3 { "Divider — Variants" }
            div { class:"demo-divider-layout",style:"display:flex;flex-direction:column;gap:var(--spacing-4);width:500px;max-width:100%",
                Divider { "data-divider-named":"showcase-subtle" }
                Divider { "data-divider-named":"showcase-strong",variant:DividerVariant::STRONG }
                Divider { "data-divider-named":"showcase-label",label:"or" }
            }
            h3 { "Divider — Full Bleed" }
            ExampleSurface {
                div { class:"demo-divider-layout",style:COLUMN,
                    Text { text_type:TextType::LABEL,"Order Summary" }
                    div { style:"display:flex;justify-content:space-between",Text{"3 items"} Text{"$127.00"} }
                    Divider { "data-divider-named":"full-items",is_full_bleed:true }
                    div { style:"display:flex;justify-content:space-between",Text{"Shipping"} Text{"$7.99"} }
                    div { style:"display:flex;justify-content:space-between",Text{"Tax"} Text{"$10.16"} }
                    Divider { "data-divider-named":"full-tax",is_full_bleed:true }
                    div { style:"display:flex;justify-content:space-between",Text{text_type:TextType::LABEL,"Total"} Text{text_type:TextType::LABEL,"$145.15"} }
                }
            }
            h3 { "Divider — Variants (card comparison)" }
            ExampleSurface {
                div { class:"demo-divider-layout",style:COLUMN,
                    div { style:"display:flex;flex-direction:column;gap:var(--spacing-1)",
                        Text{text_type:TextType::LABEL,"Sign in with email"}
                        Text{"Enter your email and password to access your account."}
                    }
                    Divider { "data-divider-named":"card-label",label:"or" }
                    div { style:"display:flex;flex-direction:column;gap:var(--spacing-1)",
                        Text{text_type:TextType::LABEL,"Sign in with SSO"}
                        Text{"Use your company credentials to sign in automatically."}
                    }
                    Divider { "data-divider-named":"card-strong",variant:DividerVariant::STRONG }
                    Text{text_type:TextType::SUPPORTING,"By signing in, you agree to our Terms of Service and Privacy Policy."}
                }
            }
            h3 { "Divider — Vertical" }
            ExampleSurface { is_intrinsic:true,
                div { class:"demo-divider-layout",style:"display:flex;align-items:stretch;gap:var(--spacing-4)",
                    for (index,title,value,change) in [(0,"Revenue","$24,500","+12% vs last month"),(1,"Users","1,240","+8% vs last month"),(2,"Conversion","3.2%","-0.5% vs last month")] {
                        div { key:"{title}",style:"flex:1 1 auto;min-width:0;display:flex;flex-direction:column;gap:var(--spacing-1)",
                            Text{text_type:TextType::LABEL,"{title}"} Text{text_type:TextType::LABEL,"{value}"}
                            Text{text_type:TextType::SUPPORTING,"{change}"}
                        }
                        if index<2 { Divider { "data-divider-named":if index==0{"vertical-revenue"}else{"vertical-users"},orientation:DividerOrientation::Vertical,style:"align-self:stretch;height:auto" } }
                    }
                }
            }
            h3 { "Labels, naming and native root inputs" }
            Divider { id:"dv-zero",label:0 }
            Divider { id:"dv-empty",label:"" }
            Divider { id:"dv-false",label:false }
            Divider { id:"dv-true",label:true }
            Divider { id:"dv-rich",label:DividerLabel::Rich(rsx!{strong{"Rich "}em{"label"}}) }
            Divider { id:"dv-named",label:"Visible", "aria-label":"Caller name" }
            span { id:"dv-external-name","External reference" }
            Divider { id:"dv-referenced",label:"Visible", "aria-label":"Other name", "aria-labelledby":"dv-external-name" }
            Divider { id:"dv-overridden",role:"note","aria-orientation":"vertical" }
            Divider { id:"dv-native",label:"Native root",lang:"fr","data-testid":"divider-native","aria-describedby":"dv-native-note",onmounted:move |_|events+=1,attributes:vec![onclick(move |_|events+=1)] }
            p { id:"dv-native-note","Root refs and event attributes remain caller-owned." }
            p { id:"dv-events","Native events: {events}" }
            Button { id:"dv-toggle-label",label:"Toggle separator label",onclick:move |_|is_labelled.toggle() }
            Divider { id:"dv-changing",label:if is_labelled(){DividerLabel::from("Stable label")}else{DividerLabel::Empty} }
            Divider { id:"dv-custom",variant:custom,label:"Custom paint is theme-owned" }
            h3 { "Logical full bleed and zero fallback" }
            for direction in ["ltr","rtl"] {
                div { id:"dv-bleed-{direction}",dir:direction,style:"box-sizing:border-box;width:320px;max-width:100%;padding-inline-start:13px;padding-inline-end:27px;--container-padding-inline-start:13px;--container-padding-inline-end:27px",
                    Divider { is_full_bleed:true,label:"Inline bleed" }
                }
                div { id:"dv-block-{direction}",dir:direction,style:"box-sizing:border-box;width:90px;height:160px;padding-block-start:11px;padding-block-end:23px;--container-padding-block-start:11px;--container-padding-block-end:23px",
                    Divider { orientation:DividerOrientation::Vertical,is_full_bleed:true,label:"Block" }
                }
            }
            div { id:"dv-zero-inset",style:"width:200px",Divider {is_full_bleed:true} }
            div { id:"dv-zero-block",style:"height:90px",Divider {orientation:DividerOrientation::Vertical,is_full_bleed:true} }
            h3 { "Both axes and weights" }
            div { style:"display:flex;flex-wrap:wrap;gap:16px",
                for (axis,orientation) in [("horizontal",DividerOrientation::Horizontal),("vertical",DividerOrientation::Vertical)] {
                    for (weight,variant) in [("subtle",DividerVariant::SUBTLE),("strong",DividerVariant::STRONG)] {
                        div { key:"{axis}-{weight}",style:"width:180px;height:100px",
                            Divider { id:"dv-{axis}-{weight}",orientation,variant,label:"Between" }
                        }
                    }
                }
            }
        }
    }
}
