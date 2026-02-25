use inquire::Text;
use inquire::validator::Validation;
use crate::error::WizardError;

pub fn run() -> Result<(String, String), WizardError> {
    let project_name = Text::new("Project name:")
        .with_help_message("Lowercase letters, digits, and underscores only (e.g. my_app)")
        .with_validator(|input: &str| {
            if input.is_empty() {
                return Ok(Validation::Invalid("Project name cannot be empty.".into()));
            }
            let starts_with_letter = input.chars().next().map(|c| c.is_ascii_lowercase()).unwrap_or(false);
            let all_valid = input.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
            if starts_with_letter && all_valid {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Only lowercase letters, digits, and underscores allowed. Must start with a letter.".into(),
                ))
            }
        })
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    let org_name = Text::new("Organization name:")
        .with_help_message("Reverse domain format (e.g. com.example)")
        .with_validator(|input: &str| {
            let parts: Vec<&str> = input.split('.').collect();
            let valid = parts.len() >= 2
                && parts.iter().all(|p| {
                    !p.is_empty()
                        && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
                });
            if valid {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Must be reverse domain format (e.g. com.example).".into(),
                ))
            }
        })
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    Ok((project_name, org_name))
}
