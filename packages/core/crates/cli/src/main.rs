use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

mod commands;
mod config_loader;

use commands::{cmd_check, cmd_init, cmd_rules};
use core::{init_logger, APP_DESCRIPTION, APP_NAME};

#[derive(Debug, Clone, ValueEnum)]
pub enum GroupMode {
    File,
    Rule,
}

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(version, about = APP_DESCRIPTION, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Scan code for issues and display results")]
    Check {
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        #[arg(long, help = "Skip cache and force full scan")]
        no_cache: bool,

        #[arg(long, help = "Group issues by rule (default: group by file)")]
        by_rule: bool,

        #[arg(long, help = "Output results as JSON")]
        json: bool,

        #[arg(long, help = "Pretty output with rule definitions at the top")]
        pretty: bool,

        #[arg(
            long,
            value_name = "BRANCH",
            help = "Only show issues in files changed compared to branch (e.g., origin/main)"
        )]
        branch: Option<String>,

        #[arg(
            long,
            value_name = "FILE_PATTERN",
            help = "Filter results to specific file(s) using glob pattern (e.g., 'src/**/*.ts')"
        )]
        file: Option<String>,

        #[arg(
            long,
            value_name = "RULE_NAME",
            help = "Filter results to specific rule (e.g., 'no-console-log')"
        )]
        rule: Option<String>,

        #[arg(long, help = "Continue execution even when errors are found")]
        continue_on_error: bool,

        #[arg(
            long,
            value_name = "CONFIG_DIR",
            help = "Path to directory containing config.jsonc"
        )]
        config: Option<PathBuf>,
    },

    #[command(about = "List all available rules and their metadata")]
    Rules {
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        #[arg(
            long,
            value_name = "CONFIG_DIR",
            help = "Path to directory containing config.jsonc"
        )]
        config: Option<PathBuf>,
    },

    #[command(about = "Create a default configuration file")]
    Init {
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    init_logger("rust_cli        ");

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Check {
            path,
            no_cache,
            by_rule,
            json,
            pretty,
            branch,
            file,
            rule,
            continue_on_error,
            config,
        }) => {
            let group_mode = if by_rule {
                GroupMode::Rule
            } else {
                GroupMode::File
            };
            cmd_check(
                &path,
                no_cache,
                group_mode,
                json,
                pretty,
                branch,
                file,
                rule,
                continue_on_error,
                config,
            )
        }
        Some(Commands::Rules { path, config }) => cmd_rules(&path, config),
        Some(Commands::Init { path }) => cmd_init(&path),
        None => {
            Cli::parse_from(["tscanner", "--help"]);
            Ok(())
        }
    }
}
