use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod commands;
mod config_loader;
mod shared;

use commands::{cmd_check, cmd_init, validate};
use tscanner_cli::{Cli, Commands};
use tscanner_service::init_logger;

fn main() -> Result<()> {
    if std::env::var("TSCANNER").map(|v| v == "0").unwrap_or(false) {
        return Ok(());
    }

    init_logger("rust_cli        ");

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Check {
            paths,
            branch,
            staged,
            uncommitted,
            include_ai,
            only_ai,
            glob,
            rule,
            severity,
            kind,
            group_by,
            format,
            json_output,
            no_cache,
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
                json_output,
                branch,
                staged,
                uncommitted,
                glob,
                rule,
                severity,
                kind,
                continue_on_error,
                include_ai,
                only_ai,
                config_path,
            )
        }
        Some(Commands::Init { full }) => cmd_init(&PathBuf::from("."), full),
        Some(Commands::Validate { config_path }) => validate(config_path),
        Some(Commands::Lsp) => {
            tscanner_service::log_info("LSP server starting");
            let result = tscanner_lsp::run_lsp_server().map_err(|e| anyhow::anyhow!("{}", e));
            match &result {
                Ok(_) => tscanner_service::log_info("LSP server exited normally"),
                Err(e) => {
                    tscanner_service::log_info(&format!("LSP server exited with error: {}", e))
                }
            }
            result
        }
        None => {
            Cli::parse_from(["tscanner", "--help"]);
            Ok(())
        }
    }
}
