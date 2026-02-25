use inquire::Confirm;
use crate::error::WizardError;
use crate::wizard::config::WizardConfig;

pub fn run(config: &WizardConfig) -> Result<bool, WizardError> {
    println!("\n========== Configuration Summary ==========");
    println!("Project name : {}", config.project_name);
    println!("Org name     : {}", config.org_name);

    let platform_names: Vec<&str> = config.platforms.iter().map(|p| p.display_name()).collect();
    println!("Platforms    : {}", platform_names.join(", "));
    println!("Architecture : {}", config.architecture.display_name());

    if config.categories.is_empty() {
        println!("Packages     : (none)");
    } else {
        println!("Packages     :");
        for cat in &config.categories {
            for pkg in &cat.packages {
                if pkg.selected_options.is_empty() {
                    println!("  - {} / {}", cat.display_name, pkg.display_name);
                } else {
                    println!(
                        "  - {} / {}  [options: {}]",
                        cat.display_name,
                        pkg.display_name,
                        pkg.selected_options.join(", ")
                    );
                }
            }
        }
    }

    println!("Environments : {}", config.environments.join(", "));

    if config.cicd_workflows.is_empty() {
        println!("CI/CD        : (none)");
    } else {
        let names: Vec<&str> = config.cicd_workflows.iter().map(|w| w.display_name()).collect();
        println!("CI/CD        : {}", names.join(", "));
    }

    match &config.license {
        Some(l) => println!("License      : {}", l.display_name()),
        None => println!("License      : (none)"),
    }

    println!("===========================================\n");

    let confirmed = Confirm::new("Proceed with project generation?")
        .with_default(true)
        .prompt()
        .map_err(|e| WizardError::Prompt(e.to_string()))?;

    Ok(confirmed)
}
