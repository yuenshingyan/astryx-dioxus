use astryx_dioxus::{Button, Skeleton, SkeletonLength, SkeletonRadius};
use dioxus::events::onclick;
use dioxus::prelude::*;

#[component]
fn NamedSkeleton(
    #[props(into)] width: SkeletonLength,
    #[props(into)] height: SkeletonLength,
    index: u32,
    identity: String,
    #[props(default)] radius: SkeletonRadius,
) -> Element {
    rsx! {Skeleton{width,height,index,radius,"data-skeleton-named":identity}}
}

#[component]
pub fn SkeletonExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut has_changed = use_signal(|| false);
    let mut is_shown = use_signal(|| false);
    let mut is_loading = use_signal(|| true);
    let mut events = use_signal(|| 0);
    rsx! {
        document::Style {"
            .demo-sk-column {{display:flex;flex-direction:column;align-items:stretch;}}
            .demo-sk-row {{display:flex;align-items:center;}}
            .demo-sk-scroll {{box-sizing:border-box;width:fit-content;max-width:100%;overflow:auto;}}
            .demo-sk-native-card {{box-sizing:border-box;width:320px;min-width:0;overflow:clip;padding:calc(var(--astryx-card-padding, var(--spacing-3)) - var(--border-width));border:var(--border-width) solid var(--color-border);border-radius:var(--radius-container);background:var(--color-background-card);}}
            .demo-sk-theme .astryx-skeleton {{width:55px;height:23px;border-radius:3px;background:rgb(30,100,180);animation:none;opacity:0.6;}}
        "}
        section {
            id:"skeleton-examples","data-astryx-theme":"neutral",
            style:if is_dark(){"color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem"}else{"color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem"},
            h2 {"Skeleton — original examples"}
            Button{id:"sk-theme",label:"Switch Skeleton light/dark",onclick:move |_|is_dark.toggle()}
            h3 {"Skeleton"}
            div {class:"demo-sk-scroll",style:"padding:var(--spacing-4);background:var(--color-background-muted)",
            div {id:"sk-showcase",class:"demo-sk-column",style:"gap:var(--spacing-4);width:300px",
                div {class:"demo-sk-row",style:"gap:var(--spacing-4)",
                    NamedSkeleton{width:64,height:64,radius:SkeletonRadius::Rounded,index:0,identity:"showcase/0"}
                    div {class:"demo-sk-column",style:"gap:var(--spacing-2)",
                        NamedSkeleton{width:160,height:20,index:1,identity:"showcase/1"}
                        NamedSkeleton{width:80,height:16,index:2,identity:"showcase/2"}
                    }
                }
                div {class:"demo-sk-column",style:"gap:var(--spacing-2)",
                    for (index,width) in [(3,"90%"),(4,"100%"),(5,"75%")] {
                        NamedSkeleton{width,height:16,index,identity:"showcase/{index}"}
                    }
                }
            }
            }
            h3 {"Skeleton — Card Loading"}
            // Native original Card/Stack framing, not a public Card/Layout port.
            div {class:"demo-sk-scroll",
            div {id:"sk-card",class:"demo-sk-native-card",
                div {class:"demo-sk-column",style:"gap:var(--spacing-3)",
                    div {class:"demo-sk-row",style:"gap:var(--spacing-3)",
                        NamedSkeleton{width:40,height:40,radius:SkeletonRadius::Rounded,index:0,identity:"card/0"}
                        div {class:"demo-sk-column",style:"gap:var(--spacing-1)",
                            NamedSkeleton{width:120,height:14,index:1,identity:"card/1"}
                            NamedSkeleton{width:80,height:12,index:2,identity:"card/2"}
                        }
                    }
                    for (index,width) in [(3,"100%"),(4,"90%"),(5,"75%")] {
                        NamedSkeleton{width,height:14,index,identity:"card/{index}"}
                    }
                }
            }
            }
            h3 {"Skeleton — Staggered List"}
            div {class:"demo-sk-scroll",
            div {id:"sk-list",class:"demo-sk-column",style:"gap:var(--spacing-2);width:max-content",
                for (index,width) in [(0,300),(1,280),(2,320),(3,260),(4,290)] {
                    NamedSkeleton{width,height:16,index,identity:"list/{index}"}
                }
            }
            }
            h3 {"Skeleton — Table Rows"}
            div {class:"demo-sk-scroll",
            div {id:"sk-table",class:"demo-sk-column",style:"gap:var(--spacing-2);width:max-content",
                for row in 0_u32..4 {
                    div {class:"demo-sk-row",style:"gap:var(--spacing-4)",
                        for (column,width) in [(0,50),(1,180),(2,100),(3,80)] {
                            NamedSkeleton{width,height:16,index:row*4+column,identity:"table/{row}/{column}"}
                        }
                    }
                }
            }
            }
            h3 {"All seven original radii"}
            div {style:"display:flex;gap:12px;flex-wrap:wrap",
                for (name,radius) in [("none",SkeletonRadius::None),("0",SkeletonRadius::Zero),("1",SkeletonRadius::One),("2",SkeletonRadius::Two),("3",SkeletonRadius::Three),("4",SkeletonRadius::Four),("rounded",SkeletonRadius::Rounded)] {
                    div {span {"{name}"} Skeleton{id:"sk-radius-{name}",width:40,height:24,radius}}
                }
            }
            h3 {"Default percentages and open CSS geometry"}
            div {style:"width:137px;height:63px",Skeleton{id:"sk-default"}}
            div {style:"width:137px;height:63px",Skeleton{id:"sk-inherit",width:"inherit",height:"inherit"}}
            div {style:"width:200px;height:20px;--sample-height:17px",Skeleton{id:"sk-css",width:"calc(100% - 2rem)",height:"var(--sample-height)"}}
            div {class:"demo-sk-theme",Skeleton{id:"sk-themed",width:200,height:30}}
            h3 {"Native caller ownership and lifecycle"}
            Skeleton {id:"sk-native",width:60,height:20,class:"caller-skeleton",style:"margin-inline:7px",lang:"fr",role:"img","aria-label":"Explicit loading preview","aria-hidden":"false","data-testid":"native-skeleton",
                onmounted:move |_|events+=1,attributes:vec![onclick(move |_|events+=1)],
            }
            p {id:"sk-events","Native events: {events}"}
            Button{id:"sk-change",label:"Change Skeleton geometry and stagger",onclick:move |_|has_changed.toggle()}
            Skeleton{id:"sk-changing",width:if has_changed(){240}else{120},height:16,radius:if has_changed(){SkeletonRadius::Rounded}else{SkeletonRadius::None},index:if has_changed(){5}else{0}}
            Button{id:"sk-toggle",label:"Mount/remove a Skeleton",onclick:move |_|is_shown.toggle()}
            if is_shown(){Skeleton{id:"sk-late",width:80,height:16,index:2}}
            Button{id:"sk-finish",label:"Replace loading shape with content",onclick:move |_|is_loading.toggle()}
            div {id:"sk-region","aria-label":"Profile preview","aria-busy":if is_loading(){"true"}else{"false"},
                if is_loading(){Skeleton{id:"sk-owned",width:80,height:16}}else{p{"Profile content is ready."}}
            }
            h3 {"CSS-only pulse"}
            div {Skeleton{id:"sk-motion",width:120,height:48}}
            h3 {"Pre-tempo duration"}
            div {style:"--duration-medium-max:4s;--_astryx-pre-tempo-duration-medium-max:700ms",Skeleton{id:"sk-pretempo",width:80,height:16}}
        }
    }
}
