use crate::error::WizardError;
use crate::packages::schema::PackageCategory;

/// All embedded YAML package definition files, bound at compile time.
const EMBEDDED_YAMLS: &[(&str, &str)] = &[
    (
        "state_management",
        include_str!("../../assets/packages/state_management.yaml"),
    ),
    (
        "database",
        include_str!("../../assets/packages/database.yaml"),
    ),
    ("di", include_str!("../../assets/packages/di.yaml")),
    (
        "data_model",
        include_str!("../../assets/packages/data_model.yaml"),
    ),
    (
        "code_generation",
        include_str!("../../assets/packages/code_generation.yaml"),
    ),
    (
        "networking",
        include_str!("../../assets/packages/networking.yaml"),
    ),
    (
        "routing",
        include_str!("../../assets/packages/routing.yaml"),
    ),
    ("logger", include_str!("../../assets/packages/logger.yaml")),
];

/// Parse all embedded YAML files and return a list of package categories.
/// Fails on the first parse error, reporting which file caused it.
pub fn load_all() -> Result<Vec<PackageCategory>, WizardError> {
    EMBEDDED_YAMLS
        .iter()
        .map(|(name, content)| {
            serde_yaml::from_str(content).map_err(|e| {
                WizardError::Config(format!("Failed to parse package YAML '{name}': {e}"))
            })
        })
        .collect()
}
