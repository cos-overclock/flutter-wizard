use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "flutter_wizard")]
#[command(about = "Interactive wizard for generating Flutter projects")]
#[command(version)]
pub struct Cli {
    /// Path to a YAML config file
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Override target directory without confirmation prompt
    #[arg(long)]
    pub force: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage default configuration
    Config(ConfigArgs),
    /// Manage project templates
    Template(TemplateArgs),
}

#[derive(Debug, clap::Args)]
pub struct ConfigArgs {
    /// Display current default configuration
    #[arg(long)]
    pub show: bool,

    /// Reset default configuration
    #[arg(long)]
    pub reset: bool,
}

#[derive(Debug, clap::Args)]
pub struct TemplateArgs {
    /// List available templates
    #[arg(long)]
    pub list: bool,

    /// Expand default templates into ~/.config/flutter_wizard/templates/
    #[arg(long)]
    pub init: bool,
}
