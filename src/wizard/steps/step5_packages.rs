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
            .ok_or_else(|| WizardError::Prompt(format!("category '{}' not found in loaded categories", cat_id)))?;

        let display: Vec<&str> = cat.packages.iter().map(|p| p.display_name.as_str()).collect();

        let selected = MultiSelect::new(
            &format!("Packages for '{}':", cat.display_name),
            display,
        )
        .with_help_message("Space to toggle, Enter to confirm.")
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

        let mut packages: Vec<SelectedPackage> = cat
            .packages
            .iter()
            .filter(|p| selected.contains(&p.display_name.as_str()))
            .map(|p| SelectedPackage {
                id: p.id.clone(),
                display_name: p.display_name.clone(),
                pub_dev_name: p.pub_dev_name.clone(),
                selected_options: vec![],
                selected_option_names: vec![],
            })
            .collect();

        // Prompt for options immediately after each package is selected.
        for sel_pkg in &mut packages {
            let orig_pkg = cat
                .packages
                .iter()
                .find(|p| p.id == sel_pkg.id)
                .ok_or_else(|| {
                    WizardError::Prompt(format!(
                        "selected package '{}' not found in loaded packages",
                        sel_pkg.id
                    ))
                })?;

            if orig_pkg.options.is_empty() {
                continue;
            }

            let opt_display: Vec<&str> = orig_pkg
                .options
                .iter()
                .map(|o| o.display_name.as_str())
                .collect();

            let chosen = MultiSelect::new(
                &format!("Options for '{}':", sel_pkg.display_name),
                opt_display,
            )
            .with_help_message("Space to toggle, Enter to confirm (leave empty to skip).")
            .prompt()
            .map_err(|e| WizardError::Prompt(e.to_string()))?;

            let chosen_opts: Vec<_> = orig_pkg
                .options
                .iter()
                .filter(|o| chosen.contains(&o.display_name.as_str()))
                .collect();
            sel_pkg.selected_options = chosen_opts.iter().map(|o| o.id.clone()).collect();
            sel_pkg.selected_option_names = chosen_opts.iter().map(|o| o.display_name.clone()).collect();
        }

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
