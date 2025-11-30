use anyhow::Result;
use clap::Parser;

mod commands;
mod config_loader;
mod shared;

use cli::{Cli, CliOverrides, Commands};
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
            hide_severity,
            hide_source_line,
            hide_rule_name,
            show_description,
            hide_summary,
            format,
            branch,
            file,
            rule,
            continue_on_error,
            config,
        }) => {
            let overrides = CliOverrides {
                group_by: group_by.clone(),
                no_cache: if no_cache { Some(true) } else { None },
                show_severity: if hide_severity { Some(false) } else { None },
                show_source_line: if hide_source_line { Some(false) } else { None },
                show_rule_name: if hide_rule_name { Some(false) } else { None },
                show_description: if show_description { Some(true) } else { None },
                show_summary_at_footer: if hide_summary { Some(false) } else { None },
            };

            cmd_check(
                &path,
                no_cache,
                format,
                branch,
                file,
                rule,
                continue_on_error,
                config,
                overrides,
            )
        }
        Some(Commands::Rules { path, config }) => cmd_rules(&path, config),
        Some(Commands::Init { path, all_rules }) => cmd_init(&path, all_rules),
        None => {
            Cli::parse_from(["tscanner", "--help"]);
            Ok(())
        }
    }
}
