use astryx_dioxus::{
    Button, ButtonVariant, Card, CardVariant, Divider, Elevation, SizeValue, SpacingStep, Text,
    TextColor, TextType, TextWeight,
};
use dioxus::events::{onclick, onmounted};
use dioxus::prelude::*;

#[component]
pub fn CardExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut has_changed = use_signal(|| false);
    let mut is_shown = use_signal(|| true);
    let mut events = use_signal(|| 0);
    let mut notice = use_signal(String::new);
    let custom = CardVariant::new("catalog_Tier-7")?;
    rsx! {
        // Native framing for the original Stack/Grid/Heading/Layout content;
        // these demonstration recipes do not complete those public migrations.
        document::Style{"
          .demo-card-context {{font-family:var(--font-family-body);font-size:var(--text-body-size);line-height:var(--text-body-leading);font-weight:400;}}
          /* Required Neutral preview typography, not a completed Theme port. */
          @media (pointer:coarse) and (width < 1024px) {{
            #card-examples[data-astryx-theme=neutral] {{
              --text-body-size:var(--font-size-base);--text-body-leading:1.5;
              --text-supporting-size:.875rem;--text-supporting-leading:1.4286;
              --text-heading-3-size:1.1875rem;--text-heading-3-leading:1.4737;
              --text-heading-4-size:var(--font-size-base);--text-heading-4-leading:1.5;
            }}
          }}
          .demo-card-column {{display:flex;flex-direction:column;align-items:stretch;}}
          .demo-card-row {{display:flex;flex-direction:row;align-items:stretch;}}
          .demo-card-scroll {{width:fit-content;max-width:100%;overflow:auto;}}
          .demo-card-heading {{margin:0;display:block;color:var(--color-text-primary);font-family:var(--font-family-heading);}}
          h3.demo-card-heading {{font-size:var(--text-heading-3-size);line-height:var(--text-heading-3-leading);font-weight:var(--text-heading-3-weight);}}
          h4.demo-card-heading {{font-size:var(--text-heading-4-size);line-height:var(--text-heading-4-leading);font-weight:var(--text-heading-4-weight);}}
          .demo-card-layout {{margin-inline-start:calc(-1 * var(--container-padding-inline-start));margin-inline-end:calc(-1 * var(--container-padding-inline-end));margin-block-start:calc(-1 * var(--container-padding-block-start));margin-block-end:calc(-1 * var(--container-padding-block-end));height:calc(100% + var(--container-padding-block-start) + var(--container-padding-block-end));}}
          .demo-card-layout-stack {{display:flex;flex-direction:column;align-items:stretch;height:100%;--container-padding-inline-start:0px;--container-padding-inline-end:0px;--container-padding-block-start:0px;--container-padding-block-end:0px;--layout-padding-own-outer-x:initial;--layout-padding-own-outer-y:initial;}}
          .demo-card-layout-header,.demo-card-layout-footer {{flex-shrink:0;}}
          .demo-card-layout-header {{border-block-end:1px solid var(--color-border);}}
          .demo-card-layout-footer {{border-block-start:1px solid var(--color-border);}}
          .demo-card-layout-inner {{box-sizing:border-box;margin-inline:auto;padding-inline:var(--layout-padding-outer-x);}}
          .demo-card-layout-header .demo-card-layout-inner {{padding-block-start:var(--layout-padding-outer-y);padding-block-end:var(--layout-padding-inner-y);}}
          .demo-card-layout-footer .demo-card-layout-inner {{padding-block-start:var(--layout-padding-inner-y);padding-block-end:var(--layout-padding-outer-y);}}
          .demo-card-layout-middle {{display:flex;flex-direction:row;align-items:stretch;flex:1;min-height:0;}}
          .demo-card-layout-lane {{flex:1;min-width:0;min-height:0;}}
          .demo-card-layout-content {{box-sizing:border-box;height:100%;flex:1;min-height:0;overflow:auto;padding-inline:var(--layout-padding-outer-x);padding-block:var(--layout-padding-inner-y);}}
          .demo-card-class-override {{width:211px;min-width:190px;padding:9px;background:rgb(100,40,160);}}
          .demo-card-theme .astryx-card[data-variant=catalog_Tier-7] {{background:rgb(30,100,180);border:2px solid currentColor;}}
          .demo-card-reader {{border-start-start-radius:var(--container-corner-start-start);border-start-end-radius:var(--container-corner-start-end);border-end-start-radius:var(--container-corner-end-start);border-end-end-radius:var(--container-corner-end-end);padding-block-start:var(--_container-bleed-block-start);padding-block-end:var(--_container-bleed-block-end);}}
        "}
        section{id:"card-examples",class:"demo-card-context","data-astryx-theme":"neutral",
            style:if is_dark(){"color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem"}else{"color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem"},
            h2{"Card — original static examples"}
            Button{id:"cd-theme",label:"Switch Card light/dark",onclick:move |_|is_dark.toggle()}
            h3{"Card"}
            div{class:"demo-card-scroll",style:"padding:var(--spacing-4);background:var(--color-background-muted)",
                Card{id:"cd-showcase",width:SizeValue::from(320),"data-card-named":"showcase",
                    div{class:"demo-card-column",style:"gap:var(--spacing-2)",
                        h4{class:"demo-card-heading","Card title"}
                        Text{color:TextColor::SECONDARY,"Cards group related content with a border and background. Use them for profiles, settings panels, or data summaries."}
                    }
                }
            }
            h3{"Card — Callout"}
            div{class:"demo-card-scroll",div{id:"cd-callout",class:"demo-card-row",style:"gap:var(--spacing-4);width:max-content",
                for (index,title,body) in [(0,"Tip","Use the muted variant for callouts or supplementary information."),(1,"Note","Muted cards work well in sidebars or help panels.")]{
                    Card{width:SizeValue::from(270),variant:CardVariant::MUTED,"data-card-named":"callout/{index}",div{class:"demo-card-column",style:"gap:var(--spacing-2)",h3{class:"demo-card-heading","{title}"}Text{color:TextColor::SECONDARY,"{body}"}}}
                }
            }}
            h3{"Card — Elevations"}
            div{class:"demo-card-scroll",div{id:"cd-elevations",style:"display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:var(--spacing-4);width:520px;max-width:100%",
                for (elevation,caption) in [(Elevation::None,"Flat — sits on the page (default)"),(Elevation::Low,"Low — lifts off slightly"),(Elevation::Med,"Med — clearly raised"),(Elevation::High,"High — floats well above")]{
                    Card{elevation,"data-card-named":"elevation/{elevation.as_str()}",div{class:"demo-card-column",style:"gap:var(--spacing-1)",h4{class:"demo-card-heading","elevation=\"{elevation.as_str()}\""}Text{text_type:TextType::SUPPORTING,color:TextColor::SECONDARY,"{caption}"}}}
                }
            }}
            h3{"Card — Variants"}
            div{class:"demo-card-scroll",div{style:"box-sizing:border-box;width:878px;padding:5px",div{id:"cd-variants",class:"demo-card-row",style:"gap:var(--spacing-3);flex-wrap:wrap",
                for (variant,label,note) in [(CardVariant::DEFAULT,"General","4 tasks due today"),(CardVariant::MUTED,"Archived","No activity in 30 days"),(CardVariant::BLUE,"Engineering","12 open issues"),(CardVariant::GREEN,"Marketing","3 campaigns active"),(CardVariant::ORANGE,"Urgent","2 items need review"),(CardVariant::PURPLE,"Design","5 drafts in progress")]{
                    Card{width:SizeValue::from(160),variant:variant.clone(),"data-card-named":"variant/{variant.as_str()}",div{class:"demo-card-column",style:"gap:var(--spacing-1)",Text{weight:TextWeight::Bold,"{label}"}Text{text_type:TextType::SUPPORTING,color:TextColor::SECONDARY,"{note}"}}}
                }
            }}
            }
            h3{"Card — Layout"}
            div{class:"demo-card-scroll",Card{id:"cd-layout",width:SizeValue::from(380),max_width:SizeValue::from("100%"),"data-card-named":"layout",
                div{class:"demo-card-layout",div{class:"demo-card-layout-stack",
                    div{class:"demo-card-layout-header",div{class:"demo-card-layout-inner",h3{class:"demo-card-heading","Edit Profile"}}}
                    div{class:"demo-card-layout-middle",div{class:"demo-card-layout-lane",div{class:"demo-card-layout-content",Text{color:TextColor::SECONDARY,"Update your display name, bio, and profile photo. Changes are saved immediately."}}}}
                    div{class:"demo-card-layout-footer",div{class:"demo-card-layout-inner",div{class:"demo-card-row",style:"gap:var(--spacing-2);justify-content:flex-end;align-items:center",
                        Button{label:"Cancel",variant:ButtonVariant::Ghost,onclick:move |_|notice.set("Demo: your edits would be discarded here".to_owned())}
                        Button{label:"Save changes",onclick:move |_|notice.set("Demo: your profile changes would be saved here".to_owned())}
                    }}}
                }}
            }}
            // The application supplies demo feedback, not a partial Toast API.
            p{id:"cd-notice",role:"status","{notice}"}
            h3{"Card — Simple"}
            div{class:"demo-card-scroll",Card{id:"cd-simple",width:SizeValue::from(360),"data-card-named":"simple",div{class:"demo-card-column",style:"gap:var(--spacing-2)",
                h3{class:"demo-card-heading","Project Overview"}
                Text{color:TextColor::SECONDARY,"This project tracks the redesign of the onboarding flow. The goal is to reduce drop-off by 15% in Q2."}
                Text{text_type:TextType::SUPPORTING,color:TextColor::SECONDARY,"Last updated 2 hours ago"}
            }}}
            h3{"Cards sharing a row on a phone, with long IDs"}
            div{id:"cd-phone",style:"display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:var(--spacing-3);width:390px;max-width:100%",
                for (label,amount,id) in [("Payout","$1,240.00","po_1Q8ZK4X7M2N5P3R6T9V1W4YB"),("Refund","-$86.50","re_3NfK9Lm2Qx7Vb4Tz8Rw1Hy6C"),("Adjustment","$12.00","adj_7Hq2Wm9Kx4Lz8Vn3Rb6Tf1PD")]{
                    Card{"data-payout-card":label,div{class:"demo-card-column",style:"gap:var(--spacing-1)",h3{class:"demo-card-heading","{label}"}Text{has_tabular_numbers:true,"{amount}"}Text{text_type:TextType::SUPPORTING,max_lines:1,"{id}"}}}
                }
            }
            h3{"Every surface and padding step"}
            div{id:"cd-all-variants",style:"display:flex;gap:8px;flex-wrap:wrap",
                for variant in [CardVariant::DEFAULT,CardVariant::TRANSPARENT,CardVariant::MUTED,CardVariant::BLUE,CardVariant::CYAN,CardVariant::GRAY,CardVariant::GREEN,CardVariant::ORANGE,CardVariant::PINK,CardVariant::PURPLE,CardVariant::RED,CardVariant::TEAL,CardVariant::YELLOW]{Card{variant:variant.clone(),width:SizeValue::from(120),"data-card-variant":variant.as_str(),Text{"{variant.as_str()}"}Text{color:TextColor::SECONDARY,text_type:TextType::SUPPORTING,"Secondary text"}}}
            }
            div{id:"cd-paddings",style:"display:flex;gap:8px;flex-wrap:wrap",
                for (padding,name) in [(SpacingStep::Zero,"0"),(SpacingStep::Half,"0-5"),(SpacingStep::One,"1"),(SpacingStep::OneAndHalf,"1-5"),(SpacingStep::Two,"2"),(SpacingStep::Three,"3"),(SpacingStep::Four,"4"),(SpacingStep::Five,"5"),(SpacingStep::Six,"6"),(SpacingStep::Eight,"8"),(SpacingStep::Ten,"10")]{Card{id:"cd-padding-{name}",width:SizeValue::from(100),padding,style:"--border-width:2px",div{style:"height:8px","{name}"}}}
            }
            h3{"Own padding, bleed and corner publication"}
            Card{id:"cd-asymmetric",width:SizeValue::from(240),height:SizeValue::from(100),style:"--astryx-card-padding-inline-start:20px;--astryx-card-padding-inline-end:8px;--astryx-card-padding-block-start:6px;--astryx-card-padding-block-end:10px;--border-width:2px",Divider{id:"cd-bleed",is_full_bleed:true}}
            Card{id:"cd-corners",width:SizeValue::from(220),div{class:"demo-card-reader","data-container-corner":"", "Direct reader"}}
            Card{id:"cd-nested-corner",width:SizeValue::from(220),div{div{class:"demo-card-reader","data-container-corner":"", "Nested reader"}}}
            Card{id:"cd-mixed-corner",width:SizeValue::from(220),div{class:"demo-card-reader","data-container-corner":"", "Direct"}div{div{class:"demo-card-reader","data-container-corner":"", "Deeper"}}}
            Card{id:"cd-nested",padding:SpacingStep::Ten,Card{id:"cd-nested-inner",width:SizeValue::from(100),"Inner card"}}
            h3{"Preferred width, clipping and fixed-height scrolling"}
            div{id:"cd-shrink",style:"display:flex;width:180px",Card{id:"cd-preferred",width:SizeValue::from(320),div{style:"white-space:nowrap","A deliberately long unbroken ID po_123456789012345678901234567890"}}}
            Card{id:"cd-scroll",width:SizeValue::from(200),height:SizeValue::from(80),div{style:"height:300px;width:500px","Overflowing content"}}
            Card{id:"cd-auto",width:SizeValue::from(200),height:SizeValue::from(" /* keyword */ \\61 uto "),div{style:"height:20px","Auto height"}}
            div{style:"width:137px;height:63px",Card{id:"cd-inherit",width:SizeValue::from("inherit"),height:SizeValue::from("inherit"),padding:SpacingStep::Zero}}
            div{style:"width:137px;height:63px",Card{id:"cd-escaped-inherit",width:SizeValue::from("\\69 nherit"),height:SizeValue::from("INITIAL"),padding:SpacingStep::Zero}}
            Card{id:"cd-css",width:SizeValue::from("calc(200px - 2rem)"),min_height:SizeValue::from("var(--sample-height, 24px)"),max_width:SizeValue::from("90vw")}
            Card{id:"cd-class",width:SizeValue::from(320),padding:SpacingStep::Four,class:"demo-card-class-override"}
            Card{id:"cd-style",width:SizeValue::from(320),class:"demo-card-class-override",style:"width:233px;padding:13px"}
            Card{id:"cd-ring",elevation:Elevation::High,style:"--_card-ring:inset 0 0 0 2px rgb(100,40,160)","Ring and elevation"}
            Card{id:"cd-fallback",variant:custom.clone(),width:SizeValue::from(120),"Custom baseline"}
            div{class:"demo-card-theme",Card{id:"cd-custom",variant:custom,width:SizeValue::from(120),"Themed custom"}}
            h3{"Native refs/events and static updates"}
            Card{id:"cd-native",width:SizeValue::from(180),class:"caller-card",style:"margin-inline:7px",lang:"fr",role:"region","aria-label":"Caller named region","data-testid":"native-card",onmounted:move |_|events+=1,attributes:vec![onclick(move |_|events+=1),onmounted(move |_|events+=1),Attribute::new("data-variant","wrong",None,false)],"Caller content"}
            p{id:"cd-events","Native events: {events}"}
            Button{id:"cd-change",label:"Change Card props",onclick:move |_|has_changed.toggle()}
            Card{id:"cd-changing",width:SizeValue::from(if has_changed(){240}else{120}),variant:if has_changed(){CardVariant::MUTED}else{CardVariant::BLUE},elevation:if has_changed(){Elevation::High}else{Elevation::None},"{has_changed}"}
            Button{id:"cd-toggle",label:"Mount/remove Card",onclick:move |_|is_shown.toggle()}
            if is_shown(){Card{id:"cd-late",width:SizeValue::from(100),"Temporary content"}}
        }
    }
}
