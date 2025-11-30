use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
pub enum GroupMode {
    File,
    Rule,
}

#[derive(Debug, Clone, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Pretty,
}

#[derive(Parser)]
#[command(name = "tscanner")]
#[command(version, about = "Code quality scanner for the AI-generated code era", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Scan code for issues and display results")]
    Check {
        #[arg(
            value_name = "PATH",
            default_value = ".",
            help = "Directory or file to scan (extra paths are ignored when --staged is used)",
            num_args = 0..
        )]
        paths: Vec<PathBuf>,

        #[arg(long, help = "Skip cache and force full scan")]
        no_cache: bool,

        #[arg(
            long,
            value_enum,
            value_name = "MODE",
            help = "Group issues by file or rule"
        )]
        group_by: Option<GroupMode>,

        #[arg(
            long,
            value_enum,
            value_name = "FORMAT",
            help = "Output format: text, json, or pretty"
        )]
        format: Option<OutputFormat>,

        #[arg(
            long,
            value_name = "BRANCH",
            help = "Only show issues in files changed compared to branch (e.g., origin/main)"
        )]
        branch: Option<String>,

        #[arg(long, help = "Scan only git staged files")]
        staged: bool,

        #[arg(
            long,
            value_name = "GLOB_PATTERN",
            help = "Filter results by glob pattern (e.g., 'src/**/*.ts')"
        )]
        glob: Option<String>,

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
        #[arg(
            long,
            value_name = "CONFIG_DIR",
            help = "Path to directory containing config.jsonc"
        )]
        config: Option<PathBuf>,
    },

    #[command(about = "Create a default configuration file")]
    Init {
        #[arg(long, help = "Initialize with all built-in rules enabled")]
        all_rules: bool,
    },
}
