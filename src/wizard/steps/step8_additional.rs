use inquire::{MultiSelect, Select};
use crate::error::WizardError;
use crate::wizard::config::{CicdWorkflow, License};

pub fn run() -> Result<(Vec<CicdWorkflow>, Option<License>), WizardError> {
    // CI/CD workflows
    let workflow_options = vec![
        CicdWorkflow::FormatAndLint,
        CicdWorkflow::Test,
        CicdWorkflow::Release,
        CicdWorkflow::StoreDistribution,
    ];
    let workflow_display: Vec<&str> = workflow_options.iter().map(|w| w.display_name()).collect();

    let selected_workflows = MultiSelect::new("CI/CD workflows:", workflow_display)
        .with_help_message("Space to toggle, Enter to confirm. Leave empty to skip.")
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    let workflows: Vec<CicdWorkflow> = workflow_options
        .into_iter()
        .filter(|w| selected_workflows.contains(&w.display_name()))
        .collect();

    // License
    let license_options = vec!["None", "MIT", "Apache 2.0", "GPL v3"];
    let selected_license = Select::new("License:", license_options)
        .with_starting_cursor(1) // default to MIT
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    let license = match selected_license {
        "MIT" => Some(License::Mit),
        "Apache 2.0" => Some(License::Apache2),
        "GPL v3" => Some(License::Gpl3),
        _ => None,
    };

    Ok((workflows, license))
}
