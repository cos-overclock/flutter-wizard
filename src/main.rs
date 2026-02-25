mod cli;
mod config;
mod error;
mod generator;
mod plugin;
mod template;
mod wizard;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Config(args)) => {
            if args.show {
                config::show()
            } else if args.reset {
                config::reset()
            } else {
                eprintln!("error: specify --show or --reset");
                std::process::exit(1);
            }
        }
        Some(Commands::Template(args)) => {
            if args.list {
                template::list()
            } else if args.init {
                template::init()
            } else {
                eprintln!("error: specify --list or --init");
                std::process::exit(1);
            }
        }
        None => wizard::run(),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
