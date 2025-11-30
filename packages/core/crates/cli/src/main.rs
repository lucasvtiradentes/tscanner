use anyhow::Result;
use clap::Parser;

mod commands;
mod config_loader;
mod shared;

use cli::{Cli, Commands};
use commands::{cmd_check, cmd_init, cmd_rules};
use core::init_logger;

fn main() -> Result<()> {
    init_logger("rust_cli        ");

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Check {
            path,
            no_cache,
            group_by,
            format,
            branch,
            glob,
            rule,
            continue_on_error,
            config,
        }) => cmd_check(
            &path,
            no_cache,
            group_by,
            format,
            branch,
            glob,
            rule,
            continue_on_error,
            config,
        ),
        Some(Commands::Rules { path, config }) => cmd_rules(&path, config),
        Some(Commands::Init { path, all_rules }) => cmd_init(&path, all_rules),
        None => {
            Cli::parse_from(["tscanner", "--help"]);
            Ok(())
        }
    }
}
