mod cli;
mod config;
mod error;
mod generator; // TODO: integrate generator::generate() after collecting user input in the wizard
mod plugin; // TODO: integrate plugin::load() into the wizard initialization once the plugin system is implemented
mod template;
mod wizard;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Config(args)) => {
            if args.show {
                config::show().map_err(|e| format!("Config error: {e}"))
            } else {
                config::reset().map_err(|e| format!("Config error: {e}"))
            }
        }
        Some(Commands::Template(args)) => {
            if args.list {
                template::list().map_err(|e| format!("Template error: {e}"))
            } else {
                template::init().map_err(|e| format!("Template error: {e}"))
            }
        }
        None => wizard::run(cli.config, cli.force).map_err(|e| format!("Wizard error: {e}")),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
