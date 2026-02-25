use serde::Deserialize;

/// A selectable option for a specific package (e.g. Riverpod's "use code generation").
#[derive(Debug, Clone, Deserialize)]
pub struct PackageOption {
    pub id: String,
    pub display_name: String,
}

/// A single installable package within a category.
#[derive(Debug, Clone, Deserialize)]
pub struct PackageDef {
    pub id: String,
    pub display_name: String,
    pub pub_dev_name: String,
    #[serde(default)]
    pub options: Vec<PackageOption>,
}

/// A full package category loaded from one YAML file.
#[derive(Debug, Clone, Deserialize)]
pub struct PackageCategory {
    pub id: String,
    pub display_name: String,
    pub packages: Vec<PackageDef>,
}
