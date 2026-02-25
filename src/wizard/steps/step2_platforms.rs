use inquire::MultiSelect;
use crate::error::WizardError;
use crate::wizard::config::Platform;

pub fn run() -> Result<Vec<Platform>, WizardError> {
    let options = vec![
        Platform::Android,
        Platform::Ios,
        Platform::Web,
        Platform::Windows,
        Platform::MacOs,
        Platform::Linux,
    ];
    let display: Vec<&str> = options.iter().map(|p| p.display_name()).collect();

    let selected = MultiSelect::new("Target platforms:", display.clone())
        .with_help_message("Space to toggle, Enter to confirm. At least one required.")
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    if selected.is_empty() {
        return Err(WizardError::Config(
            "At least one platform must be selected.".to_string(),
        ));
    }

    let result = options
        .into_iter()
        .filter(|p| selected.contains(&p.display_name()))
        .collect();

    Ok(result)
}
