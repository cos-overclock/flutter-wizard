use crate::error::WizardError;

/// List available templates in ~/.config/flutter_wizard/templates/.
pub fn list() -> Result<(), WizardError> {
    println!("template list: not yet implemented");
    Ok(())
}

/// Expand default templates into ~/.config/flutter_wizard/templates/.
pub fn init() -> Result<(), WizardError> {
    println!("template init: not yet implemented");
    Ok(())
}
