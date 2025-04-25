use commit_formatter::{
    get_default_commit_types,
    get_cm_types_from_file,
    get_optional_commit_body_and_footer,
    put_together_commit_message
};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::process::Command;

fn main() {
    let commit_types = if let Ok(types) = get_cm_types_from_file() {
        types
    } else {
        get_default_commit_types()
    };

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select a header:")
        .items(&commit_types)
        .default(0)
        .interact_opt()
        .unwrap();

    let commit_type = match selection {
        Some(index) => &commit_types[index],
        None => panic!("Must select a commit type!"),
    };

    let scope: String = Input::new()
        .with_prompt("The scope of this change")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let subject: String = Input::new()
        .with_prompt("A short description for your commit")
        .interact_text()
        .unwrap();

    let other = get_optional_commit_body_and_footer();
    let commit_message = put_together_commit_message(commit_type, scope, subject, other);

    Command::new("git")
        .args(&["commit", "-m", &commit_message])
        .status()
        .expect("Failed to git commit");
}
