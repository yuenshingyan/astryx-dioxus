use dioxus::html::FileData;
use dioxus::prelude::*;

use astryx_dioxus::FileInput;

#[component]
pub(super) fn FileInputExamples() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        section { id: "file-input-examples",
            h2 { "File Input" }
            form { enctype: "multipart/form-data",
                FileInput { label: "Upload file", width: Some(350.into()), html_name: "attachment",
                    description: "Choose a local file. This demo does not upload it.",
                    accept: ".txt", is_required: true,
                    on_change: move |files: Vec<FileData>| count.set(files.len()),
                }
                output { id: "file-input-count", "{count()} selected" }
                FileInput { label: "Attachments", html_name: "attachments", is_multiple: true }
                FileInput { label: "Unavailable upload", html_name: "unavailable", is_disabled: true }
            }
        }
    }
}
