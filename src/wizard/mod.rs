mod config;
mod steps;

use std::path::PathBuf;
use crate::error::WizardError;
use crate::packages;
use config::WizardConfig;

/// Run the interactive project generation wizard.
pub fn run(_config: Option<PathBuf>, force: bool) -> Result<(), WizardError> {
    // Load embedded package definitions once at startup.
    let categories = packages::load_all()?;

    // Step 1: Project name and org name
    let (project_name, org_name) = steps::step1_project::run()?;

    // Step 2: Target platforms
    let platforms = steps::step2_platforms::run()?;

    // Step 3: Architecture
    let architecture = steps::step3_architecture::run()?;

    // Step 4: Package categories
    let selected_category_ids = steps::step4_categories::run(&categories)?;

    // Step 5: Packages within each selected category (options prompted immediately after each package)
    let selected_categories =
        steps::step5_packages::run(&categories, selected_category_ids)?;

    // Step 7: Environment names
    let environments = steps::step7_environments::run()?;

    // Step 8: CI/CD workflows and license
    let (cicd_workflows, license) = steps::step8_additional::run()?;

    // Assemble the full configuration.
    let wizard_config = WizardConfig {
        project_name,
        org_name,
        platforms,
        architecture,
        categories: selected_categories,
        environments,
        cicd_workflows,
        license,
    };

    // Check for existing target directory.
    let target_dir = std::path::Path::new(&wizard_config.project_name);
    if target_dir.exists() && !force {
        let confirmed = inquire::Confirm::new(&format!(
            "Directory '{}' already exists. Overwrite?",
            wizard_config.project_name
        ))
        .with_default(false)
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

        if !confirmed {
            return Err(WizardError::Config(
                "Aborted: target directory already exists.".to_string(),
            ));
        }
    }

    // Step 9: Confirmation screen
    let confirmed = steps::step9_confirm::run(&wizard_config)?;
    if !confirmed {
        return Err(WizardError::Config(
            "Aborted by user at confirmation screen.".to_string(),
        ));
    }

    println!("\nWizard completed. Configuration confirmed.");
    Ok(())
}
