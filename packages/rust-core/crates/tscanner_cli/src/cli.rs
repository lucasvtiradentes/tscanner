use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use tscanner_constants::config_dir_name;

#[derive(Debug, Clone, Default, ValueEnum, PartialEq)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CliGroupMode {
    File,
    Rule,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CliSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

impl CliSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            CliSeverity::Error => "error",
            CliSeverity::Warning => "warning",
            CliSeverity::Info => "info",
            CliSeverity::Hint => "hint",
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CliRuleKind {
    Builtin,
    Regex,
    Script,
    Ai,
}

impl CliRuleKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            CliRuleKind::Builtin => "builtin",
            CliRuleKind::Regex => "regex",
            CliRuleKind::Script => "script",
            CliRuleKind::Ai => "ai",
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RegistryRuleKind {
    Ai,
    Script,
    Regex,
}

impl RegistryRuleKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RegistryRuleKind::Ai => "ai",
            RegistryRuleKind::Script => "script",
            RegistryRuleKind::Regex => "regex",
        }
    }
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
    #[command(
        about = "Scan code for issues and display results",
        after_help = "\u{200B}"
    )]
    Check {
        #[arg(
            value_name = "PATH",
            default_value = ".",
            help = "Directory or file to scan (extra paths are ignored when --staged is used)",
            num_args = 0..
        )]
        paths: Vec<PathBuf>,

        #[arg(
            long,
            value_name = "BRANCH",
            help = "Only show issues in files changed compared to branch (e.g., origin/main)",
            help_heading = "Scan Mode"
        )]
        branch: Option<String>,

        #[arg(long, help = "Scan only git staged files", help_heading = "Scan Mode")]
        staged: bool,

        #[arg(
            long,
            help = "Scan all uncommitted changes (staged + unstaged)",
            help_heading = "Scan Mode"
        )]
        uncommitted: bool,

        #[arg(
            long,
            help = "Include AI rules in the scan (slower)",
            help_heading = "AI Rules"
        )]
        include_ai: bool,

        #[arg(
            long,
            help = "Run only AI rules, skip all other rules",
            help_heading = "AI Rules"
        )]
        only_ai: bool,

        #[arg(
            long,
            value_name = "GLOB_PATTERN",
            help = "Filter results by glob pattern (e.g., 'src/**/*.ts')",
            help_heading = "Filtering"
        )]
        glob: Option<String>,

        #[arg(
            long,
            value_name = "RULE_NAME",
            help = "Filter results to specific rule (e.g., 'no-console')",
            help_heading = "Filtering"
        )]
        rule: Option<String>,

        #[arg(
            long,
            value_enum,
            value_name = "LEVEL",
            help = "Filter results by minimum severity (e.g., 'error')",
            help_heading = "Filtering"
        )]
        severity: Option<CliSeverity>,

        #[arg(
            long,
            value_enum,
            value_name = "TYPE",
            help = "Filter results by rule type (e.g., 'builtin')",
            help_heading = "Filtering"
        )]
        kind: Option<CliRuleKind>,

        #[arg(
            long,
            value_enum,
            value_name = "MODE",
            help = "Group issues by file or rule",
            help_heading = "Output"
        )]
        group_by: Option<CliGroupMode>,

        #[arg(
            long,
            value_enum,
            value_name = "FORMAT",
            default_value = "text",
            help = "Output format: text or json",
            help_heading = "Output"
        )]
        format: OutputFormat,

        #[arg(
            long,
            value_name = "FILE",
            help = "Additionally save JSON output to file (works with any format)",
            help_heading = "Output"
        )]
        json_output: Option<PathBuf>,

        #[arg(long, help = "Skip cache and force full scan", help_heading = "Other")]
        no_cache: bool,

        #[arg(
            long,
            help = "Continue execution even when errors are found",
            help_heading = "Other"
        )]
        continue_on_error: bool,

        #[arg(
            long,
            value_name = "CONFIG_DIR",
            help = "Path to config folder (defaults to .tscanner)",
            help_heading = "Other"
        )]
        config_path: Option<PathBuf>,
    },

    #[command(about = "Create a default configuration file")]
    Init {
        #[arg(
            long,
            help = "Initialize with all built-in rules, example regex/script/AI rules, and sample files"
        )]
        full: bool,
    },

    #[command(about = "Validate configuration file")]
    Validate {
        #[arg(
            value_name = "CONFIG_PATH",
            help = "Path to config file or directory (defaults to .tscanner/config.jsonc)"
        )]
        config_path: Option<PathBuf>,
    },

    #[command(about = "Start the LSP server (Language Server Protocol)")]
    Lsp,

    #[command(about = "Install rules from the TScanner registry")]
    Registry {
        #[arg(
            value_name = "NAME",
            help = "Rule name to install (shows list if omitted)"
        )]
        name: Option<String>,

        #[arg(
            long,
            value_enum,
            value_name = "KIND",
            help = "Filter by rule kind (ai, script, regex)"
        )]
        kind: Option<RegistryRuleKind>,

        #[arg(long, value_name = "CATEGORY", help = "Filter by category")]
        category: Option<String>,

        #[arg(long, help = "Overwrite existing rules")]
        force: bool,

        #[arg(
            long,
            help = "Use latest rules from main branch instead of version-matched"
        )]
        latest: bool,

        #[arg(
            long,
            value_name = "CONFIG_DIR",
            help = "Path to config folder (defaults to .tscanner)"
        )]
        config_path: Option<PathBuf>,
    },
}

impl Commands {
    pub fn get_config_path(&self) -> PathBuf {
        match self {
            Commands::Check { config_path, .. } => config_path
                .clone()
                .unwrap_or_else(|| PathBuf::from(config_dir_name())),
            _ => PathBuf::from(config_dir_name()),
        }
    }
}
