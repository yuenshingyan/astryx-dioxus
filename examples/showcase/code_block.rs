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
        }
    }
}
