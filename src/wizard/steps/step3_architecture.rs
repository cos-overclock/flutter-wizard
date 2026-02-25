use inquire::Select;
use crate::error::WizardError;
use crate::wizard::config::Architecture;

pub fn run() -> Result<Architecture, WizardError> {
    let options = vec![
        Architecture::CleanArchitecture,
        Architecture::Mvvm,
        Architecture::Mvc,
        Architecture::LayeredArchitecture,
    ];
    let display: Vec<&str> = options.iter().map(|a| a.display_name()).collect();

    let selected = Select::new("Architecture:", display)
        .without_filtering()
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    let arch = options
        .into_iter()
        .find(|a| a.display_name() == selected)
        .expect("selected value must exist in options");

    Ok(arch)
}
