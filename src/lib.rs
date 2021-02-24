use dialoguer::{Confirm, Editor};
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone)]
pub struct CommitType {
    text: &'static str,
    description: &'static str,
}

impl CommitType {
    pub fn default_commit_types() -> [CommitType; 8] {
        [
            CommitType {
                text: "feat",
                description: "A new feature"
            },
            CommitType {
                text: "fix",
                description: "A bug fix"
            },
            CommitType {
                text: "docs",
                description: "Documentation only changes"
            },
            CommitType {
                text: "style",
                description: "Changes that do not affect the meaning of the code(white-space, fomatting, missing semi-colons, etc)"
            },
            CommitType {
                text: "refactor",
                description: "A code change that neither fixes a bug or adds a feature"
            },
            CommitType {
                text: "perf",
                description: "A code change that improves performance"
            },
            CommitType {
                text: "test",
                description: "Adding missing tests"
            },
            CommitType {
                text: "chore",
                description: "Change to the build process or auxiliary tools and libraries such as documentation generation"
            },
        ]
    }
}

impl Display for CommitType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:9}: {}", self.text, self.description)
    }
}

pub fn get_optional_commit_body_and_footer() -> Option<String> {
    let should_open_editor = Confirm::new()
        .with_prompt("Do you want to write a long description?")
        .default(false)
        .show_default(false)
        .interact()
        .unwrap();
    if should_open_editor {
        return Editor::new().edit("").unwrap();
    }
    None
}

pub fn put_together_commit_message(
    commit_type: CommitType,
    scope: String,
    subject: String,
    optional_body_and_footer: Option<String>,
) -> String {
    let mut format_commit_message = commit_type.text.to_string();
    if scope.is_empty() {
        format_commit_message.push_str(": ");
    } else {
        format_commit_message.push_str(&format!("({}): ", scope));
    }
    format_commit_message.push_str(&subject);
    if let Some(text) = optional_body_and_footer {
        format_commit_message.push_str(&format!("\n\n{}", text));
    }
    format_commit_message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_to_string() {
        let fix = CommitType {
            text: "fix",
            description: "just for test",
        };
        assert_eq!(fix.to_string(), String::from("fix      : just for test"));
    }

    #[test]
    fn test_composite_commit() {
        let bug = CommitType {
            text: "bug",
            description: "a test",
        };
        let scope = String::from("view");
        let subject = String::from("test example");
        let other: Option<String> = None;
        let result = put_together_commit_message(bug, scope, subject, other);
        assert_eq!(result, String::from("bug(view): test example"))
    }
}
