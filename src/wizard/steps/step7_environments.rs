use inquire::{MultiSelect, Text};
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

    // Phase 2: add custom environment names
    loop {
        let extra = Text::new("Add a custom environment name (leave empty to finish):")
            .prompt()
            .map_err(|e| WizardError::Prompt(e.to_string()))?;

        let trimmed = extra.trim().to_string();
        if trimmed.is_empty() {
            break;
        }
        if !envs.contains(&trimmed) {
            envs.push(trimmed);
        }
    }

    if envs.is_empty() {
        return Err(WizardError::Config(
            "At least one environment must be configured.".to_string(),
        ));
    }

    Ok(envs)
}
