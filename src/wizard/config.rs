/// Tracks which options the user selected for a specific package.
#[derive(Debug, Clone)]
pub struct SelectedPackage {
    pub id: String,
    pub display_name: String,
    pub pub_dev_name: String,
    /// Subset of PackageOption.id values the user chose.
    pub selected_options: Vec<String>,
}

/// Tracks which packages within a category were selected.
#[derive(Debug, Clone)]
pub struct SelectedCategory {
    pub id: String,
    pub display_name: String,
    pub packages: Vec<SelectedPackage>,
}

/// The complete set of user choices collected across all wizard steps.
#[derive(Debug, Clone)]
pub struct WizardConfig {
    // Step 1
    pub project_name: String,
    pub org_name: String,

    // Step 2
    pub platforms: Vec<Platform>,

    // Step 3
    pub architecture: Architecture,

    // Steps 4, 5, 6
    pub categories: Vec<SelectedCategory>,

    // Step 7
    pub environments: Vec<String>,

    // Step 8
    pub cicd_workflows: Vec<CicdWorkflow>,
    pub license: Option<License>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Android,
    Ios,
    Web,
    Windows,
    MacOs,
    Linux,
}

impl Platform {
    pub fn display_name(&self) -> &'static str {
        match self {
            Platform::Android => "Android",
            Platform::Ios => "iOS",
            Platform::Web => "Web",
            Platform::Windows => "Windows",
            Platform::MacOs => "macOS",
            Platform::Linux => "Linux",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Architecture {
    CleanArchitecture,
    Mvvm,
    Mvc,
    LayeredArchitecture,
}

impl Architecture {
    pub fn display_name(&self) -> &'static str {
        match self {
            Architecture::CleanArchitecture => "Clean Architecture",
            Architecture::Mvvm => "MVVM",
            Architecture::Mvc => "MVC",
            Architecture::LayeredArchitecture => "Layered Architecture",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CicdWorkflow {
    FormatAndLint,
    Test,
    Release,
    StoreDistribution,
}

impl CicdWorkflow {
    pub fn display_name(&self) -> &'static str {
        match self {
            CicdWorkflow::FormatAndLint => {
                "Format and lint check (flutter analyze / dart format)"
            }
            CicdWorkflow::Test => "Test execution (flutter test)",
            CicdWorkflow::Release => "Release creation (GitHub Releases)",
            CicdWorkflow::StoreDistribution => "Store distribution (Google Play / App Store)",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum License {
    Mit,
    Apache2,
    Gpl3,
}

impl License {
    pub fn display_name(&self) -> &'static str {
        match self {
            License::Mit => "MIT",
            License::Apache2 => "Apache 2.0",
            License::Gpl3 => "GPL v3",
        }
    }
}
