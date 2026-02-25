use clap::{Parser, Subcommand, builder::Styles, builder::styling::AnsiColor};
use std::path::PathBuf;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().bold())
    .usage(AnsiColor::BrightGreen.on_default().bold())
    .literal(AnsiColor::BrightGreen.on_default())
    .placeholder(AnsiColor::White.on_default());

#[derive(Debug, Parser)]
#[command(name = "flutter_wizard")]
#[command(about = "Interactive wizard for generating Flutter projects")]
#[command(version)]
#[command(styles = STYLES)]
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
