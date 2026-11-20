use astryx_dioxus::{
    Button, ButtonVariant, CardVariant, ClickableCard, Elevation, LinkProvider, LinkRenderProps,
    SizeValue, Text, TextColor,
};
use dioxus::events::{onclick, oncontextmenu, onkeydown, onmounted, onmouseup, onpointerdown};
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, prelude::wasm_bindgen};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(
    inline_js = "export function isCapturedOrdinaryEvent(event) { return event === window.clickableOriginalEvent; }"
)]
extern "C" {
    #[wasm_bindgen(js_name = isCapturedOrdinaryEvent)]
    fn is_captured_ordinary_event(event: &web_sys::MouseEvent) -> bool;
}

fn native_target(event: &MouseEvent) -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(native) = event.data().downcast::<web_sys::MouseEvent>() {
        return native
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .map(|element| element.tag_name())
            .unwrap_or_default();
    }
    let _ = event;
    String::new()
}

fn native_trace(event: &MouseEvent) -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(native) = event.data().downcast::<web_sys::MouseEvent>() {
        let point = event.client_coordinates();
        let target = native
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .map(|target| target.tag_name())
            .unwrap_or_default();
        return format!(
            "same:{};trusted:{};owner:{};target:{};native:{},{};typed:{},{}",
            is_captured_ordinary_event(native),
            native.is_trusted(),
            native_target(event),
            target,
            native.client_x(),
            native.client_y(),
            point.x,
            point.y
        );
    }
    let _ = event;
    String::new()
}

fn mounted_tag(event: &MountedEvent) -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(element) = event.data().downcast::<web_sys::Element>() {
        return element.tag_name();
    }
    let _ = event;
    String::new()
}

#[component]
pub fn ClickableCardExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut is_disabled = use_signal(|| false);
    let mut is_cancelled = use_signal(|| false);
    let mut is_shown = use_signal(|| true);
    let mut has_native_builder = use_signal(|| true);
    let mut dynamic_events = use_signal(String::new);
    let mut count = use_signal(|| 0);
    let mut up = use_signal(|| 0);
    let mut pointer = use_signal(|| 0);
    let mut target = use_signal(String::new);
    let mut native_event_trace = use_signal(String::new);
    let mut notice = use_signal(String::new);
    let mut order = use_signal(String::new);
    let mut raw = use_signal(String::new);
    let mut mounted_order = use_signal(String::new);
    let handler = EventHandler::new(move |event: MouseEvent| {
        count += 1;
        target.set(native_target(&event));
        native_event_trace.set(native_trace(&event));
        if is_cancelled() {
            event.prevent_default();
            event.stop_propagation();
        }
    });
    rsx! {
        document::Style{"
          #clickable-examples * {{box-sizing:border-box;}}
          .demo-clickable-column {{display:flex;flex-direction:column;align-items:stretch;}}
          .demo-clickable-heading {{margin:0;display:block;color:var(--color-text-primary);font-family:var(--font-family-heading);font-size:var(--text-heading-4-size);line-height:var(--text-heading-4-leading);font-weight:var(--text-heading-4-weight);}}
          #clickable-examples {{font-family:var(--font-family-body);font-size:var(--text-body-size);line-height:var(--text-body-leading);}}
        "}
        section{id:"clickable-examples","data-astryx-theme":"neutral",style:if is_dark(){"color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:24px"}else{"color-scheme:light;background:#fff;color:var(--color-text-primary);padding:24px"},
            h2{"ClickableCard — original examples"}
            Button{id:"cc-theme",label:"Switch ClickableCard light/dark",onclick:move |_|is_dark.toggle()}
            h3{"ClickableCardShowcase"}
            ClickableCard{id:"cc-showcase",label:"Settings",href:"#",width:SizeValue::from(320),"data-clickable-named":"showcase",
                div{class:"demo-clickable-column",style:"gap:var(--spacing-2)",h4{class:"demo-clickable-heading","Settings"}Text{color:TextColor::SECONDARY,"Click anywhere on this card to navigate. Nested buttons and links work independently."}}
            }
            h3{"ClickableCardElevated"}
            ClickableCard{id:"cc-elevated",label:"Open report",href:"#",elevation:Elevation::Med,width:SizeValue::from(320),"data-clickable-named":"elevated",
                div{class:"demo-clickable-column",style:"gap:var(--spacing-2)",h4{class:"demo-clickable-heading","Quarterly report"}Text{color:TextColor::SECONDARY,"A raised shadow signals the whole card is clickable, lifting it above the surrounding content."}}
            }
            h3{"ClickableCardWithNestedButton"}
            ClickableCard{id:"cc-nested-example",label:"Product",href:"#",width:SizeValue::from(320),"data-clickable-named":"nested",
                div{class:"demo-clickable-column",style:"gap:var(--spacing-3)",
                    div{class:"demo-clickable-column",style:"gap:var(--spacing-1)",h4{class:"demo-clickable-heading","Wireless Headphones"}Text{color:TextColor::SECONDARY,"$79.99"}}
                    Button{label:"Add to cart",variant:ButtonVariant::Primary,onclick:move |_|notice.set("Demo: Wireless Headphones would be added to your cart here".into())}
                }
            }
            p{id:"cc-notice",role:"status","{notice}"}
            h3{"Native boundary and dynamic states"}
            Button{id:"cc-disable",label:"Toggle disabled",onclick:move |_|is_disabled.toggle()}
            Button{id:"cc-cancel",label:"Toggle callback cancellation",onclick:move |_|is_cancelled.toggle()}
            Button{id:"cc-toggle",label:"Remove/restore card",onclick:move |_|is_shown.toggle()}
            output{id:"cc-count","Actions: {count}; target: {target}; mouseup: {up}; pointerdown: {pointer}"}
            output{id:"cc-native-event","{native_event_trace}"}
            if is_shown(){
                ClickableCard{id:"cc-action",label:"Native action",is_disabled:is_disabled(),onclick:handler,onmouseup:move |_|up+=1,onpointerdown:move |_|pointer+=1,width:SizeValue::from(320),
                    span{id:"cc-surface","Selectable surface text"}
                    button{id:"cc-child",r#type:"button",onclick:move |_|notice.set("Nested control".into()),"Nested control"}
                    input{id:"cc-input","aria-label":"Independent input"}
                    div{id:"cc-role",role:"switch",tabindex:0,"Role switch"}
                    div{id:"cc-readonly",role:"button","aria-readonly":"true","Read only role"}
                    div{id:"cc-layer","data-astryx-layer-root":"",role:"tooltip",button{id:"cc-layer-button",r#type:"button","Layer control"}}
                }
            }
            ClickableCard{id:"cc-link",label:"Navigation",href:"#cc-destination",is_disabled:is_disabled(),onclick:handler,"Navigation surface"}
            ClickableCard{id:"cc-blank",label:"New tab",href:"#cc-destination",target:"_blank",onclick:handler,"New tab surface"}
            ClickableCard{id:"cc-blocked",label:"Blocked destination",href:"java\nscript:alert(1)",onclick:handler,"Blocked destination surface"}
            ClickableCard{id:"cc-noop",label:"Released no-op","Focusable no-op"}
            LinkProvider{component:Callback::new(move |props:LinkRenderProps|rsx!{a{href:props.href,target:props.target,rel:props.rel,onclick:move |event:MouseEvent|{props.onclick.call(event.clone());event.prevent_default();notice.set("Router navigation".into());},onmounted:move |event|props.onmounted.call(event),..props.attributes,{props.children}}}),
                ClickableCard{id:"cc-router",label:"Router card",href:"/local-route",is_disabled:is_disabled(),onclick:handler,"Router surface"}
            }
            output{id:"cc-order-log","{order}"}
            output{id:"cc-raw-log","{raw}"}
            output{id:"cc-mounted-log","{mounted_order}"}
            Button{id:"cc-clear-order",label:"Clear order log",onclick:move |_|{order.set(String::new());raw.set(String::new());}}
            ClickableCard{id:"cc-order",label:"Child order",href:"#cc-order-destination",is_disabled:is_disabled(),
                onclick:move |event|{order.write().push_str("root|");target.set(native_target(&event));},
                onmouseup:move |_|order.write().push_str("root-up|"),
                onpointerdown:move |_|order.write().push_str("root-pointer|"),
                onmounted:move |event|mounted_order.write().push_str(&format!("mounted:{}|",mounted_tag(&event))),
                attributes:vec![
                    onclick(move |_|order.write().push_str("raw|")),
                    onmouseup(move |_|order.write().push_str("raw-up|")),
                    onpointerdown(move |_|order.write().push_str("raw-pointer|")),
                    onmounted(move |_|mounted_order.write().push_str("raw-mounted|")),
                ],
                span{id:"cc-prevent",onclick:move |event|{order.write().push_str("child|");event.prevent_default();},"Prevent destination"}
                span{id:"cc-stop",onclick:move |event|{order.write().push_str("child-stop|");event.stop_propagation();},"Stop bubbling"}
                span{id:"cc-up-child",onmouseup:move |_|order.write().push_str("child-up|"),"Mouseup child"}
                Button{id:"cc-order-button",label:"Independent ordered button",onclick:move |_|order.write().push_str("button|")}
            }
            ClickableCard{id:"cc-raw",label:"Reserved and native builders",is_disabled:is_disabled(),attributes:vec![onclick(move |_|raw.write().push_str("reserved-click|")),onmouseup(move |_|raw.write().push_str("reserved-up|")),oncontextmenu(move |event:MouseEvent|{event.prevent_default();raw.write().push_str("context|");}),onkeydown(move |_|raw.write().push_str("key|"))],"Native builder surface"}
            Button{id:"cc-toggle-builder",label:"Toggle non-reserved native builder",onclick:move |_|has_native_builder.toggle()}
            output{id:"cc-dynamic-events","{dynamic_events}"}
            ClickableCard{id:"cc-dynamic",label:"Dynamic native events",is_disabled:is_disabled(),
                onclick:move |_|dynamic_events.write().push_str("click|"),
                onpointerdown:move |_|dynamic_events.write().push_str("pointer|"),
                attributes:if has_native_builder(){vec![onkeydown(move |_|dynamic_events.write().push_str("key|"))]}else{vec![]},
                "Dynamic non-reserved builder"
            }
            div{id:"cc-order-destination","Order destination"}
            div{id:"cc-variants",style:"display:flex;flex-wrap:wrap;gap:8px",
                for variant in [CardVariant::DEFAULT,CardVariant::TRANSPARENT,CardVariant::MUTED,CardVariant::BLUE,CardVariant::CYAN,CardVariant::GRAY,CardVariant::GREEN,CardVariant::ORANGE,CardVariant::PINK,CardVariant::PURPLE,CardVariant::RED,CardVariant::TEAL,CardVariant::YELLOW,CardVariant::new("custom_Tier-7")?]{
                    ClickableCard{label:variant.as_str(),variant:variant.clone(),onclick:handler,width:SizeValue::from(120),"{variant.as_str()}"}
                }
            }
            div{id:"cc-destination","Local navigation destination"}
        }
    }
}
