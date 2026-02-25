use thiserror::Error;

#[derive(Debug, Error)]
pub enum WizardError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Generator error: {0}")]
    Generator(String),

    #[error("Prompt error: {0}")]
    Prompt(String),
}
