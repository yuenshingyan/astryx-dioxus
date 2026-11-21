//! Status labels reported by AvatarStatusDot to its owning Avatar.

use std::collections::BTreeMap;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct AvatarStatusLabelContext(pub(crate) Option<Signal<BTreeMap<usize, String>>>);

pub(super) fn use_report_status_label(label: Option<String>) {
    let owner = try_consume_context::<AvatarStatusLabelContext>().and_then(|context| context.0);
    let id = current_scope_id().0;
    use_effect(use_reactive(&label, move |label| {
        if let Some(mut owner) = owner {
            let label = label.filter(|label| !label.trim().is_empty());
            if owner.peek().get(&id) != label.as_ref() {
                if let Some(label) = label {
                    owner.write().insert(id, label);
                } else {
                    owner.write().remove(&id);
                }
            }
        }
    }));
    use_drop(move || {
        if let Some(mut owner) = owner
            && let Ok(mut labels) = owner.try_write()
        {
            labels.remove(&id);
        }
    });
}
