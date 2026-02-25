use std::path::PathBuf;
use crate::error::WizardError;

/// Run the interactive project generation wizard.
pub fn run(_config: Option<PathBuf>, _force: bool) -> Result<(), WizardError> {
    println!("wizard: not yet implemented");
    Ok(())
}
