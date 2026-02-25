use inquire::MultiSelect;
use crate::error::WizardError;
use crate::packages::PackageCategory;

/// Returns a list of category IDs the user selected (may be empty to skip all).
pub fn run(categories: &[PackageCategory]) -> Result<Vec<String>, WizardError> {
    let display: Vec<&str> = categories.iter().map(|c| c.display_name.as_str()).collect();

    let selected = MultiSelect::new("Package categories:", display)
        .with_help_message("Space to toggle, Enter to confirm. Leave empty to skip all packages.")
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    let ids = categories
        .iter()
        .filter(|c| selected.contains(&c.display_name.as_str()))
        .map(|c| c.id.clone())
        .collect();

    Ok(ids)
}
