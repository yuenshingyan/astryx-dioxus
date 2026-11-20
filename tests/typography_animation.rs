use std::cell::Cell;
use std::rc::Rc;

use astryx_dioxus::{Heading, HeadingLevel, Text};
use dioxus::core::{AttributeValue, Mutation};
use dioxus::html::events::onanimationiteration as iteration_attribute;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
enum Target {
    Text,
    Heading,
}

#[test]
fn animation_listeners_are_opt_in_and_diff_independently_of_raw_attributes() {
    for target in [Target::Text, Target::Heading] {
        let phase = Rc::new(Cell::new(0));
        let render_phase = phase.clone();
        let mut dom = VirtualDom::new_with_props(
            move |phase: Rc<Cell<u8>>| {
                let phase = phase.get();
                let start = (phase == 1 || phase == 2).then(|| EventHandler::new(|_| {}));
                let iteration = (phase == 2).then(|| EventHandler::new(|_| {}));
                let end = (phase == 2).then(|| EventHandler::new(|_| {}));
                let mut attributes =
                    vec![Attribute::new("data-phase", phase.to_string(), None, false)];
                if phase == 3 {
                    attributes.push(iteration_attribute(|_| {}));
                }
                // An event inserted before this raw attribute must not cause its
                // removal when that event disappears on the next render.
                attributes.push(Attribute::new("id", "animation-target", None, false));
                match target {
                    Target::Text => rsx! {
                        Text { onanimationstart: start, onanimationiteration: iteration,
                            onanimationend: end, attributes, "Text" }
                    },
                    Target::Heading => rsx! {
                        Heading { level: HeadingLevel::Two,
                            onanimationstart: start, onanimationiteration: iteration,
                            onanimationend: end, attributes, "Heading" }
                    },
                }
            },
            render_phase,
        );
        let initial = dom.rebuild_to_vec();
        assert!(animation_registrations(&initial.edits).is_empty());
        for (next, expected) in [
            (1, vec!["animationstart"]),
            (2, vec!["animationend", "animationiteration"]),
            (3, vec!["animationiteration"]),
            (4, vec![]),
            (
                2,
                vec!["animationend", "animationiteration", "animationstart"],
            ),
        ] {
            phase.set(next);
            dom.mark_dirty(ScopeId::APP);
            let updates = dom.render_immediate_to_vec();
            assert_eq!(animation_registrations(&updates.edits), expected);
            assert!(!updates.edits.iter().any(|edit| matches!(
                edit,
                Mutation::SetAttribute {
                    name: "id",
                    value: AttributeValue::None,
                    ..
                }
            )));
            let html = dioxus_ssr::render(&dom);
            assert!(html.contains(&format!("data-phase=\"{next}\"")), "{html}");
        }
    }
}

fn animation_registrations(edits: &[Mutation]) -> Vec<&str> {
    let mut names: Vec<_> = edits
        .iter()
        .filter_map(|edit| match edit {
            Mutation::NewEventListener { name, .. } if name.starts_with("animation") => {
                Some(name.as_str())
            }
            _ => None,
        })
        .collect();
    names.sort_unstable();
    names
}
