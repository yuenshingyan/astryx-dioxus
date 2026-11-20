use astryx_dioxus::{
    Button, ProgressBar, ProgressBarMark, ProgressBarState, ProgressBarVariant, Text, TextColor,
    TextType,
};
use dioxus::events::{onclick, onmounted};
use dioxus::prelude::*;

#[component]
fn NamedProgress(
    label: String,
    identity: String,
    #[props(default)] state: ProgressBarState,
    #[props(default)] variant: ProgressBarVariant,
    #[props(default)] has_value_label: bool,
    #[props(default)] format_value_label: Option<Callback<(f64, f64), String>>,
) -> Element {
    rsx! {ProgressBar{label,state,variant,has_value_label,format_value_label,"data-progress-bar-named":identity}}
}

#[component]
pub fn ProgressBarExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut value = use_signal(|| 10.0);
    let mut is_larger_range = use_signal(|| false);
    let mut are_marks_reversed = use_signal(|| false);
    let mut marks = vec![
        ProgressBarMark::new(0.0, "Start"),
        ProgressBarMark::new(20.0, "First target"),
        ProgressBarMark::new(80.0, "Goal"),
    ];
    if are_marks_reversed() {
        marks.reverse();
    }
    let mut is_indeterminate = use_signal(|| false);
    let mut is_shown = use_signal(|| true);
    let mut is_alt_locale = use_signal(|| false);
    let mut events = use_signal(|| 0);
    let custom = ProgressBarVariant::new("queued")?;
    let percentage_template = if is_alt_locale() {
        "{percent}\u{a0}٪"
    } else {
        "{percent}%"
    };
    rsx! {
        document::Style{"
            .demo-pg-context {{font-family:var(--font-family-body);font-size:var(--text-body-size);line-height:var(--text-body-leading);font-weight:400;}}
            .demo-pg-column {{display:flex;flex-direction:column;gap:var(--spacing-1);width:300px;}}
            .demo-pg-scroll {{width:fit-content;max-width:100%;overflow:auto;}}
            .demo-pg-custom .astryx-progress-bar-fill[data-variant=queued] {{background:rgb(100,40,160);}}
            .demo-pg-tall .astryx-progress-bar-mark {{height:16px;width:4px;}}
        "}
        section{id:"progress-bar-examples",class:"demo-pg-context","data-astryx-theme":"neutral",
            style:if is_dark(){"color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem"}else{"color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem"},
            h2{"ProgressBar — original examples"}
            Button{id:"pg-theme",label:"Switch ProgressBar light/dark",onclick:move |_|is_dark.toggle()}
            p{"Source colors remain. Approved inset boundaries distinguish completed and remaining progress, including where a track matches its backdrop."}
            h3{"ProgressBar"}
            div{class:"demo-pg-scroll",style:"background:var(--color-background-muted);padding:var(--spacing-4)",
                div{id:"pg-showcase",style:"width:300px",NamedProgress{label:"Progress",identity:"showcase",state:ProgressBarState::value(60.0)}}
            }
            h3{"ProgressBar — Custom Format"}
            div{id:"pg-custom-format",style:"width:300px",div{class:"demo-pg-column",
                NamedProgress{label:"Disk usage",identity:"custom",state:ProgressBarState::range(3.2,5.0),has_value_label:true,format_value_label:move |(value,max):(f64,f64)|format!("{value} GB / {max} GB")}
                Text{text_type:TextType::SUPPORTING,color:TextColor::SECONDARY,"1.8 GB remaining"}
            }
            }
            h3{"ProgressBar — Indeterminate"}
            div{id:"pg-indeterminate",style:"width:300px",NamedProgress{label:"Loading...",identity:"indeterminate",state:ProgressBarState::Indeterminate}}
            h3{"ProgressBar — Semantic Variants"}
            div{style:"display:flex;width:100%;overflow:auto",div{style:"min-width:fit-content",
            div{id:"pg-variants",class:"demo-pg-column",style:"gap:var(--spacing-4);width:100%;max-width:300px",
                for (variant,label,value) in [(ProgressBarVariant::ACCENT,"Accent",60.0),(ProgressBarVariant::SUCCESS,"Positive",80.0),(ProgressBarVariant::WARNING,"Warning",50.0),(ProgressBarVariant::ERROR,"Negative",92.0),(ProgressBarVariant::NEUTRAL,"Neutral",35.0)]{
                    NamedProgress{variant,label,identity:format!("variant-{label}"),state:ProgressBarState::value(value),has_value_label:true}
                }
            }
            }}
            h3{"ProgressBar — With Value Label"}
            div{id:"pg-value-label",style:"width:300px",NamedProgress{label:"Storage used",identity:"value-label",state:ProgressBarState::value(75.0),has_value_label:true}}
            h3{"Theme-timed updates and target marks"}
            div{class:"showcase-row",
                Button{id:"pg-to-90",label:"Set progress to 90",onclick:move |_|value.set(90.0)}
                Button{id:"pg-to-10",label:"Set progress to 10",onclick:move |_|value.set(10.0)}
                Button{id:"pg-to-zero",label:"Set progress to zero",onclick:move |_|value.set(0.0)}
                Button{id:"pg-to-one",label:"Set progress to one",onclick:move |_|value.set(1.0)}
                Button{id:"pg-range",label:"Change progress range",onclick:move |_|is_larger_range.toggle()}
                Button{id:"pg-reorder",label:"Reverse target order",onclick:move |_|are_marks_reversed.toggle()}
            }
            ProgressBar{id:"pg-changing",label:"Controlled progress",style:"width:300px",state:ProgressBarState::marked(value(),if is_larger_range(){200.0}else{100.0},marks),has_value_label:true}
            ProgressBar{id:"pg-narrow",label:"Narrow progress",style:"width:48px",state:ProgressBarState::marked(value(),100.0,vec![ProgressBarMark::new(10.0,"Minimum-covered target"),ProgressBarMark::new(80.0,"Far target")])}
            div{class:"demo-pg-tall",ProgressBar{id:"pg-tall",label:"Tall themed marks",style:"width:300px",state:ProgressBarState::marked(45.0,100.0,vec![ProgressBarMark::new(20.0,"Reached"),ProgressBarMark::new(80.0,"Goal")])}}
            ProgressBar{id:"pg-mark-bounds",label:"Clamped target positions",style:"width:300px",state:ProgressBarState::marked(50.0,100.0,vec![ProgressBarMark::new(-10.0,"Before range"),ProgressBarMark::new(200.0,"After range"),ProgressBarMark::new(f64::NAN,""),ProgressBarMark::new(f64::INFINITY,"")])}
            h3{"Hidden labels, disabled state and safe range normalization"}
            ProgressBar{id:"pg-hidden",label:"Accessible hidden label",style:"width:300px",is_label_hidden:true,state:ProgressBarState::value(25.0)}
            ProgressBar{id:"pg-hidden-value",label:"Hidden label with value",style:"width:300px",is_label_hidden:true,has_value_label:true,state:ProgressBarState::value(25.0)}
            ProgressBar{id:"pg-disabled",label:"Canceled operation",style:"width:300px",is_disabled:true,state:ProgressBarState::marked(35.0,100.0,vec![ProgressBarMark::new(20.0,"Past target"),ProgressBarMark::new(80.0,"Future target")]),has_value_label:true}
            ProgressBar{id:"pg-nonfinite",label:"Unknown numeric input",style:"width:300px",state:ProgressBarState::range(f64::NAN,f64::INFINITY),has_value_label:true}
            ProgressBar{id:"pg-negative-max",label:"Invalid range normalized",style:"width:300px",state:ProgressBarState::range(5.0,-10.0),has_value_label:true}
            h3{"Localized default message and theme extensions"}
            Button{id:"pg-locale",label:"Change percentage message",onclick:move |_|is_alt_locale.toggle()}
            ProgressBar{id:"pg-localized",label:"Localized percentage",style:"width:300px",state:ProgressBarState::value(42.0),has_value_label:true,percentage_template}
            ProgressBar{id:"pg-fallback",label:"Unthemed custom variant",style:"width:300px",variant:custom.clone(),state:ProgressBarState::marked(60.0,100.0,vec![ProgressBarMark::new(30.0,"Custom target")])}
            div{class:"demo-pg-custom",ProgressBar{id:"pg-custom",label:"Painted custom variant",style:"width:300px",variant:custom,state:ProgressBarState::value(60.0)}}
            h3{"Native props, mode changes and lifetime"}
            ProgressBar{id:"pg-native",label:"Native progress",state:ProgressBarState::value(25.0),class:"caller-progress",style:"width:280px;margin-inline:7px",lang:"fr",role:"group","aria-describedby":"pg-description","data-testid":"native-progress",onmounted:move |_|events+=1,attributes:vec![onclick(move |_|events+=1),onmounted(move |_|events+=1),Attribute::new("data-variant","wrong",None,false)]}
            p{id:"pg-description","Application-owned supplementary description."}
            p{id:"pg-events","Native events: {events}"}
            Button{id:"pg-mode",label:"Toggle progress mode",onclick:move |_|is_indeterminate.toggle()}
            ProgressBar{id:"pg-mode-changing",label:"Changing progress mode",style:"width:300px",state:if is_indeterminate(){ProgressBarState::Indeterminate}else{ProgressBarState::marked(60.0,100.0,vec![ProgressBarMark::new(80.0,"Goal")])},has_value_label:true}
            Button{id:"pg-toggle",label:"Mount/remove progress",onclick:move |_|is_shown.toggle()}
            if is_shown(){ProgressBar{id:"pg-drop",label:"Temporary progress",style:"width:300px",state:ProgressBarState::Indeterminate}}
            h3{"Nearest rendered direction and busy-loop timing"}
            div{dir:"rtl",
                ProgressBar{id:"pg-ltr-in-rtl",dir:"ltr",label:"LTR region in RTL page",style:"width:300px",state:ProgressBarState::marked(25.0,100.0,vec![ProgressBarMark::new(20.0,"Reached"),ProgressBarMark::new(80.0,"Goal")])}
                ProgressBar{id:"pg-ind-ltr-in-rtl",dir:"ltr",label:"LTR indeterminate region",style:"width:300px",state:ProgressBarState::Indeterminate}
                ProgressBar{id:"pg-rtl",label:"RTL progress",style:"width:300px",state:ProgressBarState::marked(25.0,100.0,vec![ProgressBarMark::new(20.0,"Reached"),ProgressBarMark::new(80.0,"Goal")])}
                ProgressBar{id:"pg-ind-rtl",label:"RTL indeterminate progress",style:"width:300px",state:ProgressBarState::Indeterminate}
            }
            div{style:"--_astryx-motion-tempo:3;--duration-medium:777ms",ProgressBar{id:"pg-tempo",label:"Unscaled busy loop",style:"width:300px",state:ProgressBarState::Indeterminate}}
        }
    }
}
