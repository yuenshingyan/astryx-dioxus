use astryx_dioxus::{Button, StatusDot, StatusDotVariant, Text};
use dioxus::events::{onclick, onkeydown};
use dioxus::prelude::*;

#[component]
fn NamedStatusDot(
    variant: StatusDotVariant,
    label: String,
    identity: String,
    #[props(default)] is_pulsing: bool,
) -> Element {
    rsx! {StatusDot{variant,label,is_pulsing,"data-status-dot-named":identity}}
}

fn mark(index: usize) -> Element {
    let path = [
        "M1 4L3 6L7 2",
        "M4 1L7 7H1Z",
        "M2 2L6 6M6 2L2 6",
        "M4 2V2.1M4 3.5V6",
        "M2 4H6",
    ][index];
    rsx! {svg{width:"8",height:"8",view_box:"0 0 8 8",fill:"none",stroke:"currentColor",stroke_width:"1",path{d:path}}}
}

#[component]
pub fn StatusDotExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut is_pulsing = use_signal(|| false);
    let mut is_shown = use_signal(|| true);
    let mut has_tooltip = use_signal(|| true);
    let mut has_updated_tip = use_signal(|| false);
    let mut is_inside = use_signal(|| false);
    let mut events = use_signal(|| 0);
    let mut host_actions = use_signal(|| 0);
    let custom = StatusDotVariant::new("queued")?;
    let tones = [
        StatusDotVariant::SUCCESS,
        StatusDotVariant::WARNING,
        StatusDotVariant::ERROR,
        StatusDotVariant::ACCENT,
        StatusDotVariant::NEUTRAL,
    ];
    rsx! {
        document::Style {"
            .demo-sd-context {{font-family:var(--font-family-body);font-size:var(--text-body-size);line-height:var(--text-body-leading);font-weight:400;}}
            #sd-indicators > .demo-sd-row {{width:auto;}}
            .demo-sd-row {{display:flex;align-items:center;gap:var(--spacing-2);width:max-content;}}
            .demo-sd-column {{display:flex;flex-direction:column;gap:var(--spacing-2);width:max-content;}}
            .demo-sd-scroll {{width:fit-content;max-width:100%;overflow:auto;}}
            .demo-sd-custom .astryx-status-dot[data-variant=queued] {{background:rgb(100,40,160);color:white;}}
            .demo-sd-hit-row {{display:flex;align-items:center;gap:48px;padding:40px;width:max-content;}}
            .demo-sd-close-row {{display:flex;align-items:center;gap:8px;padding:40px;width:max-content;}}
        "}
        section {id:"status-dot-examples",class:"demo-sd-context","data-astryx-theme":"neutral",
            style:if is_dark(){"color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem"}else{"color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem"},
            h2 {"StatusDot — original examples"}
            Button{id:"sd-theme",label:"Switch StatusDot light/dark",onclick:move |_|is_dark.toggle()}
            p {"Original color-only patterns follow below. Applications must also convey status with visible text or distinct shapes; accessible names alone do not help sighted color-blind users."}
            h3 {"Status Dot"}
            div {class:"demo-sd-scroll",style:"background:var(--color-background-muted);padding:var(--spacing-4)",
                div {id:"sd-showcase",class:"demo-sd-row",
                    for (index,(variant,label)) in tones.clone().into_iter().zip(["Positive","Warning","Negative","Info","Neutral"]).enumerate() {
                        NamedStatusDot{variant,label,identity:"showcase/{index}"}
                    }
                }
            }
            h3 {"StatusDot — Pulsing"}
            div {id:"sd-pulsing",class:"demo-sd-row",
                for (index,(variant,label)) in tones.clone().into_iter().zip(["Live","Processing","Error","Processing","Error"]).enumerate() {
                    NamedStatusDot{variant,label,is_pulsing:true,identity:"pulsing/{index}"}
                }
            }
            h3 {"StatusDot — Status Indicators"}
            div {id:"sd-indicators",class:"demo-sd-column",
                for (index,(variant,label)) in [(StatusDotVariant::SUCCESS,"Online"),(StatusDotVariant::WARNING,"Away"),(StatusDotVariant::ERROR,"Offline"),(StatusDotVariant::NEUTRAL,"Unknown")].into_iter().enumerate() {
                    div {class:"demo-sd-row",NamedStatusDot{variant,label,identity:"indicators/{index}"} Text{"{label}"}}
                }
            }
            h3 {"StatusDot — Variants"}
            div {id:"sd-variants",class:"demo-sd-row",
                for (index,(variant,label)) in tones.clone().into_iter().zip(["Positive","Warning","Negative","Info","Neutral"]).enumerate() {
                    NamedStatusDot{variant,label,identity:"variants/{index}"}
                }
            }
            h3 {"Non-color marks (caller artwork)"}
            div {class:"demo-sd-row",
                for (index,variant) in tones.into_iter().enumerate() {
                    StatusDot{id:"sd-icon-{index}",variant,label:format!("Marked status {index}"),icon:mark(index)}
                }
            }
            div {class:"demo-sd-row",
                StatusDot{id:"sd-empty-text",variant:StatusDotVariant::NEUTRAL,label:"Empty text icon",icon:""}
                StatusDot{id:"sd-false",variant:StatusDotVariant::NEUTRAL,label:"False icon",icon:false}
                StatusDot{id:"sd-true",variant:StatusDotVariant::NEUTRAL,label:"True icon",icon:true}
                StatusDot{id:"sd-zero",variant:StatusDotVariant::NEUTRAL,label:"Zero icon",icon:0}
            }
            h3 {"Approved custom fallback and theme paint"}
            StatusDot{id:"sd-fallback",variant:custom.clone(),label:"Queued"}
            div {class:"demo-sd-custom",StatusDot{id:"sd-custom",variant:custom,label:"Queued in custom theme"}}
            h3 {"Native props and static/dynamic pulse"}
            StatusDot{id:"sd-native",variant:StatusDotVariant::SUCCESS,label:"Native status",class:"caller-dot",style:"margin-inline:7px",lang:"fr","data-testid":"native-status-dot",
                onmounted:move |_|events+=1,attributes:vec![onclick(move |_|events+=1)],
            }
            p {id:"sd-events","Native events: {events}"}
            StatusDot{id:"sd-override",variant:StatusDotVariant::NEUTRAL,label:"Default name",role:"note","aria-label":"Caller-owned name",tabindex:"-1"}
            Button{id:"sd-change",label:"Toggle StatusDot pulse",onclick:move |_|is_pulsing.toggle()}
            StatusDot{id:"sd-changing",variant:StatusDotVariant::WARNING,label:"Changing activity",is_pulsing:is_pulsing()}
            h3 {"Standalone explanatory tooltip"}
            StatusDot{id:"sd-tip",variant:StatusDotVariant::SUCCESS,label:"Online",tooltip:"Connected to the workspace"}
            Button{id:"sd-toggle-tip",label:"Toggle optional StatusDot tooltip",onclick:move |_|has_tooltip.toggle()}
            StatusDot{id:"sd-optional",variant:StatusDotVariant::NEUTRAL,label:"Optional details",tooltip:has_tooltip().then(||"Additional explanation".to_owned())}
            StatusDot{id:"sd-cancel",variant:StatusDotVariant::NEUTRAL,label:"Canceled control",tooltip:"Must not pin after caller cancellation",attributes:vec![onclick(|event:MouseEvent|event.prevent_default()),onkeydown(|event:KeyboardEvent|event.prevent_default())]}
            div {id:"sd-pin-boundary",onclick:move |_|host_actions+=100,
                StatusDot{id:"sd-contained",variant:StatusDotVariant::ACCENT,label:"Independent status toggle",tooltip:"This press belongs to the status control"}
            }
            h3 {"Inside native controls: no nested toggle"}
            button {id:"sd-host-button",onclick:move |_|host_actions+=1,"Publish " StatusDot{id:"sd-in-button",variant:StatusDotVariant::SUCCESS,label:"Online",tooltip:"Connection verified"}}
            a {id:"sd-host-link",href:"#status-dot-examples","Dashboard " StatusDot{id:"sd-in-link",variant:StatusDotVariant::WARNING,label:"Processing",tooltip:"Data is updating"}}
            for (id,role) in [("menu","menuitem"),("option","option"),("role-button","button"),("role-link","link")] {
                // Native ancestor-context probes, not public Menu/Listbox ports.
                div {role:match role{"menuitem"=>Some("menu"),"option"=>Some("listbox"),_=>None},"aria-label":matches!(role,"menuitem"|"option").then_some("Native status host example"),
                    div {id:"sd-host-{id}",role,tabindex:"0","aria-selected":(role=="option").then_some("false"),"Host {role} " StatusDot{id:"sd-in-{id}",variant:StatusDotVariant::NEUTRAL,label:"Ready",tooltip:"Status explanation"}}
                }
            }
            button {id:"sd-labelled-host","aria-label":"Manage users — Online","Manage " StatusDot{id:"sd-in-labelled",variant:StatusDotVariant::SUCCESS,label:"Online",tooltip:"Connection detail"}}
            p {id:"sd-host-actions","Host actions: {host_actions}"}
            h3 {"Placement, content and lifetime changes"}
            Button{id:"sd-place",label:"Toggle interactive ancestor",onclick:move |_|is_inside.toggle()}
            div {id:"sd-dynamic-host",role:if is_inside(){"button"}else{"presentation"},"Dynamic host " StatusDot{id:"sd-dynamic",variant:StatusDotVariant::SUCCESS,label:"Ready",tooltip:"Dynamic status detail"}}
            Button{id:"sd-update",label:"Update explanation",onclick:move |_|has_updated_tip.toggle()}
            button {id:"sd-updating-host","Save " StatusDot{id:"sd-updating",variant:StatusDotVariant::SUCCESS,label:"Online",tooltip:if has_updated_tip(){"Updated connection detail"}else{"Original connection detail"}}}
            Button{id:"sd-toggle",label:"Mount/remove tooltip dot",onclick:move |_|is_shown.toggle()}
            if is_shown(){StatusDot{id:"sd-drop",variant:StatusDotVariant::ERROR,label:"Offline",tooltip:"Reconnect required"}}
            h3 {"Roomy and tightly spaced touch targets"}
            div {class:"demo-sd-scroll",div {class:"demo-sd-hit-row",id:"sd-roomy",
                StatusDot{id:"sd-hit-a",variant:StatusDotVariant::SUCCESS,label:"A",tooltip:"First roomy target"}
                StatusDot{id:"sd-hit-b",variant:StatusDotVariant::WARNING,label:"B",tooltip:"Second roomy target"}
                button {id:"sd-hit-neighbor",style:"width:16px;height:16px;padding:0","N"}
            }}
            div {class:"demo-sd-scroll",div {class:"demo-sd-close-row",id:"sd-close",
                StatusDot{id:"sd-close-a",variant:StatusDotVariant::SUCCESS,label:"A",tooltip:"First close target"}
                StatusDot{id:"sd-close-b",variant:StatusDotVariant::WARNING,label:"B",tooltip:"Second close target"}
            }}
        }
    }
}
