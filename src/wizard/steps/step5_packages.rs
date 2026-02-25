use inquire::MultiSelect;
use crate::error::WizardError;
use crate::packages::PackageCategory;
use crate::wizard::config::{SelectedCategory, SelectedPackage};

pub fn run(
    all_categories: &[PackageCategory],
    selected_ids: Vec<String>,
) -> Result<Vec<SelectedCategory>, WizardError> {
    let mut result = Vec::new();

    for cat_id in &selected_ids {
        let cat = all_categories
            .iter()
            .find(|c| &c.id == cat_id)
            .expect("category id must exist in loaded categories");

        let display: Vec<&str> = cat.packages.iter().map(|p| p.display_name.as_str()).collect();

        let selected = MultiSelect::new(
            &format!("Packages for '{}':", cat.display_name),
            display,
        )
        .with_help_message("Space to toggle, Enter to confirm.")
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

        let packages: Vec<SelectedPackage> = cat
            .packages
            .iter()
            .filter(|p| selected.contains(&p.display_name.as_str()))
            .map(|p| SelectedPackage {
                id: p.id.clone(),
                display_name: p.display_name.clone(),
                pub_dev_name: p.pub_dev_name.clone(),
                selected_options: vec![],
            })
            .collect();

        if !packages.is_empty() {
            result.push(SelectedCategory {
                id: cat.id.clone(),
                display_name: cat.display_name.clone(),
                packages,
            });
        }
    }

    Ok(result)
}
