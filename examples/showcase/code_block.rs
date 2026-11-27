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
    rsx! {
        section { id: "code-block-examples",
            h2 { "Code Block" }
            CodeBlock { code: SOURCE, title: "useUser.ts".to_owned(), has_line_numbers: true, has_copy_button: true }
            div { id: "code-block-diff-example",
                h3 { "Unified diff" }
                CodeBlock { code: "--- a/settings.rs\n+++ b/settings.rs\n@@ -1,2 +1,2 @@\n-const RETRIES: u8 = 2;\n+const RETRIES: u8 = 3;\n const DELAY: u8 = 5;", is_diff: true, has_line_numbers: true, has_copy_button: true }
            }
        }
    }
}
