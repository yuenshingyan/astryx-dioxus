use dioxus::prelude::*;

use astryx_dioxus::CodeBlock;

const SOURCE: &str = r#"import {useState, useEffect} from 'react';

export function useUser(id: string) {
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    fetch(`/api/users/${id}`)
      .then(res => res.json())
      .then(setUser);
  }, [id]);

  return user;
}"#;

#[component]
pub(super) fn CodeBlockExamples() -> Element {
    let mut is_saved = use_signal(|| false);
    rsx! {
        section { id: "code-block-examples",
            h2 { "Code Block" }
            CodeBlock { code: SOURCE, title: "useUser.ts".to_owned(), has_line_numbers: true, has_copy_button: true }
            div { id: "code-block-disclosure-example",
                h3 { "Collapsible source" }
                CodeBlock { code: SOURCE, title: "Source excerpt", is_collapsible: true, has_copy_button: true,
                    end_content: rsx! { button { r#type: "button", onclick: move |_| is_saved.toggle(), if is_saved() { "Saved" } else { "Save excerpt" } } },
                }
            }
            div { id: "code-block-corner-example",
                h3 { "Headerless source" }
                CodeBlock { code: "let answer = 42;", has_copy_button: true,
                    end_content: rsx! { a { href: "#code-block-disclosure-example", "See full excerpt" } },
                }
            }
            div { id: "code-block-diff-example",
                h3 { "Unified diff" }
                CodeBlock { code: "--- a/settings.rs\n+++ b/settings.rs\n@@ -1,2 +1,2 @@\n-const RETRIES: u8 = 2;\n+const RETRIES: u8 = 3;\n const DELAY: u8 = 5;", is_diff: true, has_line_numbers: true, has_copy_button: true }
            }
        }
    }
}
