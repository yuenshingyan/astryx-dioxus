//! Browser-only test harness for the internal native boundary (not public API).
#[cfg(target_arch = "wasm32")]
#[path = "../src/layer/layer_surface_press.rs"]
pub(crate) mod layer_surface_press;
#[cfg(target_arch = "wasm32")]
#[path = "../src/utils/safe_url.rs"]
pub(crate) mod safe_url;
#[cfg(target_arch = "wasm32")]
#[path = "../src/hooks/use_clickable_container.rs"]
mod use_clickable_container;
#[cfg(target_arch = "wasm32")]
mod layer {
    pub(crate) use super::layer_surface_press;
}
#[cfg(target_arch = "wasm32")]
#[path = "../src/private_attributes/mod.rs"]
mod private_attributes;
#[cfg(target_arch = "wasm32")]
mod utils {
    pub(crate) use super::safe_url;
}

#[cfg(target_arch = "wasm32")]
mod probe {
    use std::cell::Cell;
    use std::rc::Rc;

    use dioxus::prelude::*;
    use wasm_bindgen::{JsCast, prelude::wasm_bindgen};
    use web_sys::{Element as DomElement, HtmlElement, MouseEvent as DomMouseEvent};

    use super::use_clickable_container::native::{
        allows_activation, apply_cancellation, delegate_action, mouse_event, navigate,
        pointer_event, use_clickable_container, use_native_document_listener, use_native_listener,
        with_current_target,
    };

    use super::use_clickable_container::use_native_owner;

    #[wasm_bindgen(
        inline_js = "export function throwForOwnerProbe() { throw new Error('native owner probe'); }"
    )]
    extern "C" {
        #[wasm_bindgen(js_name = throwForOwnerProbe)]
        fn throw_for_owner_probe();
    }

    #[component]
    pub fn App() -> Element {
        let mut is_shown = use_signal(|| true);
        rsx! {button{id:"probe-toggle",onclick:move |_|is_shown.toggle(),"Toggle"} if is_shown(){Probe{}}}
    }

    #[component]
    fn Probe() -> Element {
        let count = use_hook(|| Rc::new(Cell::new(0)));
        let (owner, owner_ref) = use_native_owner();
        let mut saved_mount = use_signal(|| None::<MountedEvent>);
        let mut is_new_label = use_signal(|| false);
        let capture_owner = owner.clone();
        let label = if is_new_label() { "outer2|" } else { "outer|" };
        let routed_capture = use_native_document_listener(
            "route-probe",
            true,
            Rc::new(move |event: web_sys::Event| {
                let root = capture_owner.element().unwrap();
                if !event
                    .target()
                    .and_then(|target| target.dyn_into::<DomElement>().ok())
                    .is_some_and(|target| root.contains(Some(&target)))
                {
                    return;
                }
                if root.get_attribute("data-reenter").as_deref() == Some("armed") {
                    root.set_attribute("data-reenter", "done").unwrap();
                    let unrelated = root
                        .owner_document()
                        .unwrap()
                        .get_element_by_id("native-unrelated")
                        .unwrap();
                    unrelated
                        .dispatch_event(
                            &DomMouseEvent::new_with_mouse_event_init_dict("route-probe", &{
                                let init = web_sys::MouseEventInit::new();
                                init.set_bubbles(true);
                                init
                            })
                            .unwrap(),
                        )
                        .unwrap();
                }
                root.set_attribute(
                    "data-route",
                    &(root.get_attribute("data-route").unwrap_or_default() + label),
                )
                .unwrap();
            }),
        );
        let bubble_owner = owner.clone();
        let routed_bubble = use_native_document_listener(
            "route-bubble",
            false,
            Rc::new(move |_| {
                let root = bubble_owner.element().unwrap();
                root.set_attribute(
                    "data-route",
                    &(root.get_attribute("data-route").unwrap_or_default() + "outer|"),
                )
                .unwrap();
            }),
        );
        let owner_scope = use_native_listener(
            "#native-probe".into(),
            "owner-probe",
            false,
            EventHandler::new(move |event: web_sys::Event| {
                let root = owner.element().unwrap();
                let own = root.query_selector("#probe-own").unwrap().unwrap();
                let outer_event = event.clone();
                let outer_owner = own.clone();
                let inner_owner = root.clone();
                let status = root.clone();
                with_current_target(&event, &own, move || {
                    assert_eq!(
                        outer_event.current_target(),
                        Some(outer_owner.clone().into())
                    );
                    let inner_event = outer_event.clone();
                    let inner_check = inner_owner.clone();
                    with_current_target(&outer_event, &inner_owner, move || {
                        assert_eq!(inner_event.current_target(), Some(inner_check.into()));
                    })
                    .unwrap();
                    assert_eq!(outer_event.current_target(), Some(outer_owner.into()));
                    status
                        .set_attribute("data-scope", "nested-restored")
                        .unwrap();
                })
                .unwrap();
                let exception = with_current_target(&event, &root, throw_for_owner_probe);
                root.set_attribute(
                    "data-exception",
                    if exception.is_err() {
                        "restored"
                    } else {
                        "missing"
                    },
                )
                .unwrap();
            }),
        );
        let click = use_clickable_container(
            "#native-probe".into(),
            Rc::new(move |event| {
                let root: DomElement = event.current_target().unwrap().unchecked_into();
                let native: &DomMouseEvent = event.unchecked_ref();
                if !allows_activation(native, &root, false) {
                    return;
                }
                count.set(count.get() + 1);
                root.set_attribute("data-count", &count.get().to_string())
                    .unwrap();
                let own: HtmlElement = root
                    .query_selector("#probe-own")
                    .unwrap()
                    .unwrap()
                    .unchecked_into();
                // Exercises normal synchronous delegation back through this SAME Fn.
                delegate_action(native, &own);
                navigate(native, "javascript:blocked()", None, None);
            }),
        );
        let capture = use_native_listener(
            "#native-probe".into(),
            "click",
            true,
            EventHandler::new(move |event: web_sys::Event| {
                let root: DomElement = event.current_target().unwrap().unchecked_into();
                let native: &DomMouseEvent = event.unchecked_ref();
                let wrapped = mouse_event(native);
                let is_original = wrapped.data().downcast::<DomMouseEvent>() == Some(native);
                wrapped.prevent_default();
                wrapped.stop_propagation();
                root.set_attribute(
                    "data-capture",
                    &format!(
                        "{is_original}:{}:{}:{}",
                        !wrapped.default_action_enabled(),
                        !wrapped.propagates(),
                        native.default_prevented()
                    ),
                )
                .unwrap();
                // Intentionally leave native metadata untouched. Caller policy is not
                // part of this transport; a layer child must still see its own click.
            }),
        );
        let pointer = use_native_listener(
            "#native-probe".into(),
            "pointerdown",
            false,
            EventHandler::new(move |event: web_sys::Event| {
                let root: DomElement = event.current_target().unwrap().unchecked_into();
                let native: &web_sys::PointerEvent = event.unchecked_ref();
                let wrapped = pointer_event(native);
                let is_original =
                    wrapped.data().downcast::<web_sys::PointerEvent>() == Some(native);
                wrapped.prevent_default();
                wrapped.stop_propagation();
                apply_cancellation(&wrapped, &event);
                root.set_attribute(
                    "data-pointer",
                    &format!(
                        "{is_original}:{}:{}",
                        native.default_prevented(),
                        wrapped.pointer_id()
                    ),
                )
                .unwrap();
            }),
        );
        rsx! {div{id:"native-probe","data-pressable-container":"true",onmounted:move |event:MountedEvent|{owner_ref.call(event.clone());owner_scope.call(event.clone());click.call(event.clone());capture.call(event.clone());pointer.call(event.clone());saved_mount.set(Some(event));},
            span{id:"probe-surface","Surface"}
            button{id:"probe-own",r#type:"button","data-astryx-container-action":"","Own action"}
            button{id:"probe-child",r#type:"button","Child"}
            span{id:"probe-readonly",role:"button","aria-readonly":"true","Read-only"}
            button{id:"probe-register-outer",onclick:move |_|{if let Some(event)=saved_mount(){routed_capture.call(event.clone());routed_bubble.call(event);}},"Register outer after inner"}
            button{id:"probe-label",onclick:move |_|is_new_label.toggle(),"Update callback label"}
            InnerProbe{}
        } UnrelatedProbe{} }
    }

    #[component]
    fn InnerProbe() -> Element {
        let route = |event: web_sys::Event| {
            let root = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("native-probe")
                .unwrap();
            if !event
                .target()
                .and_then(|target| target.dyn_into::<DomElement>().ok())
                .is_some_and(|target| root.contains(Some(&target)))
            {
                return;
            }
            root.set_attribute(
                "data-route",
                &(root.get_attribute("data-route").unwrap_or_default() + "inner|"),
            )
            .unwrap();
        };
        let capture = use_native_document_listener("route-probe", true, Rc::new(route));
        let bubble = use_native_document_listener("route-bubble", false, Rc::new(route));
        rsx! {div{id:"native-inner",onmounted:move |event:MountedEvent|{capture.call(event.clone());bubble.call(event);},span{id:"probe-inner-surface","Inner surface"}}}
    }

    #[component]
    fn UnrelatedProbe() -> Element {
        let capture = use_native_document_listener("route-probe", true, Rc::new(|_| {}));
        let bubble = use_native_document_listener("route-bubble", false, Rc::new(|_| {}));
        rsx! {div{id:"native-unrelated",onmounted:move |event:MountedEvent|{capture.call(event.clone());bubble.call(event);},"Unrelated registration between inner and outer"}}
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(probe::App);
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
