use inquire::MultiSelect;
use crate::error::WizardError;
use crate::packages::PackageCategory;
use crate::wizard::config::SelectedCategory;

pub fn run(
    mut selected: Vec<SelectedCategory>,
    all_categories: &[PackageCategory],
) -> Result<Vec<SelectedCategory>, WizardError> {
    for sel_cat in &mut selected {
        let orig_cat = all_categories
            .iter()
            .find(|c| c.id == sel_cat.id)
            .expect("selected category must exist in loaded categories");

        for sel_pkg in &mut sel_cat.packages {
            let orig_pkg = orig_cat
                .packages
                .iter()
                .find(|p| p.id == sel_pkg.id)
                .expect("selected package must exist in loaded packages");

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

            sel_pkg.selected_options = orig_pkg
                .options
                .iter()
                .filter(|o| chosen.contains(&o.display_name.as_str()))
                .map(|o| o.id.clone())
                .collect();
        }
    }

    Ok(selected)
}
