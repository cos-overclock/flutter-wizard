use inquire::{MultiSelect, Select, Text};
use crate::error::WizardError;

const DEFAULT_ENVS: &[&str] = &["develop", "staging", "production"];

pub fn run() -> Result<Vec<String>, WizardError> {
    // Phase 1: let user keep/discard default environment names
    let kept = MultiSelect::new("Select environments to include:", DEFAULT_ENVS.to_vec())
        .with_default(&[0, 1, 2])
        .with_help_message("Defaults: develop, staging, production. Deselect to remove.")
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    let mut envs: Vec<String> = kept.iter().map(|s| s.to_string()).collect();

    // Phase 2: add or remove custom environment names
    loop {
        let mut choices = vec!["Add a custom environment", "Remove a custom environment", "Done"];
        // Only show "Remove" when there are custom envs to remove
        let custom_count = envs.iter().filter(|e| !DEFAULT_ENVS.contains(&e.as_str())).count();
        if custom_count == 0 {
            choices.retain(|&c| c != "Remove a custom environment");
        }

        let action = Select::new("Custom environments:", choices)
            .with_help_message("Add or remove custom environment names, then select Done.")
            .prompt()
            .map_err(|e| WizardError::Prompt(e.to_string()))?;

        match action {
            "Add a custom environment" => {
                let extra = Text::new("New environment name:")
                    .prompt()
                    .map_err(|e| WizardError::Prompt(e.to_string()))?;
                let trimmed = extra.trim().to_string();
                if !trimmed.is_empty() && !envs.contains(&trimmed) {
                    envs.push(trimmed);
                }
            }
            "Remove a custom environment" => {
                let custom_envs: Vec<String> = envs
                    .iter()
                    .filter(|e| !DEFAULT_ENVS.contains(&e.as_str()))
                    .cloned()
                    .collect();
                let to_remove = Select::new("Select environment to remove:", custom_envs)
                    .prompt()
                    .map_err(|e| WizardError::Prompt(e.to_string()))?;
                envs.retain(|e| e != &to_remove);
            }
            _ => break,
        }
    }

    if envs.is_empty() {
        return Err(WizardError::Config(
            "At least one environment must be configured.".to_string(),
        ));
    }

    Ok(envs)
}
