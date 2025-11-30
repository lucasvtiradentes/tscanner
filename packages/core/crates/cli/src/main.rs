use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod commands;
mod config_loader;
mod shared;

use cli::{Cli, Commands};
use commands::{cmd_check, cmd_config, cmd_init};
use core::init_logger;

fn main() -> Result<()> {
    if std::env::var("TSCANNER").map(|v| v == "0").unwrap_or(false) {
        return Ok(());
    }

    init_logger("rust_cli        ");

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Check {
            paths,
            no_cache,
            group_by,
            format,
            branch,
            staged,
            glob,
            rule,
            continue_on_error,
            config_path,
        }) => {
            let paths = if paths.is_empty() {
                vec![PathBuf::from(".")]
            } else {
                paths
            };
            cmd_check(
                &paths,
                no_cache,
                group_by,
                Some(format),
                branch,
                staged,
                glob,
                rule,
                continue_on_error,
                Some(config_path),
            )
        }
        Some(Commands::Config {
            rules,
            validate,
            show,
            config_path,
        }) => cmd_config(
            &PathBuf::from("."),
            rules,
            validate,
            show,
            Some(config_path),
        ),
        Some(Commands::Init { all_rules }) => cmd_init(&PathBuf::from("."), all_rules),
        None => {
            Cli::parse_from(["tscanner", "--help"]);
            Ok(())
        }
    }
}
