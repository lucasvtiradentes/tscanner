use clap::CommandFactory;
use cli::Cli;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CliOutput {
    name: String,
    version: String,
    description: String,
    commands: Vec<CommandInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommandInfo {
    name: String,
    description: String,
    usage: String,
    arguments: Vec<ArgumentInfo>,
    flags: Vec<FlagInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ArgumentInfo {
    name: String,
    description: Option<String>,
    required: bool,
    default_value: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FlagInfo {
    name: String,
    short: Option<char>,
    description: Option<String>,
    takes_value: bool,
    value_name: Option<String>,
    default_value: Option<String>,
    required: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Cli::command();

    let name = cmd.get_name().to_string();
    let version = cmd.get_version().unwrap_or("unknown").to_string();
    let description = cmd.get_about().map(|s| s.to_string()).unwrap_or_default();

    let mut commands = Vec::new();

    for subcmd in cmd.get_subcommands() {
        if subcmd.get_name() == "help" {
            continue;
        }

        let mut arguments = Vec::new();
        let mut flags = Vec::new();

        for arg in subcmd.get_arguments() {
            let arg_name = arg.get_id().to_string();

            if arg_name == "help" || arg_name == "version" {
                continue;
            }

            let is_positional = arg.is_positional();
            let description = arg.get_help().map(|s| s.to_string());
            let default_value = arg
                .get_default_values()
                .first()
                .map(|v| v.to_string_lossy().to_string());
            let required = arg.is_required_set();
            let value_name = arg
                .get_value_names()
                .and_then(|names| names.first().map(|n| n.to_string()));

            let kebab_name = arg_name.replace('_', "-");

            if is_positional {
                arguments.push(ArgumentInfo {
                    name: kebab_name,
                    description,
                    required,
                    default_value,
                });
            } else {
                let short = arg.get_short();
                let takes_value = arg.get_action().takes_values();

                flags.push(FlagInfo {
                    name: kebab_name,
                    short,
                    description,
                    takes_value,
                    value_name: if takes_value { value_name } else { None },
                    default_value,
                    required,
                });
            }
        }

        flags.sort_by(|a, b| a.name.cmp(&b.name));

        let usage = format!(
            "{} {} [OPTIONS]{}",
            name,
            subcmd.get_name(),
            if arguments.is_empty() {
                String::new()
            } else {
                format!(
                    " {}",
                    arguments
                        .iter()
                        .map(|a| if a.required {
                            format!("<{}>", a.name.to_uppercase())
                        } else {
                            format!("[{}]", a.name.to_uppercase())
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
        );

        commands.push(CommandInfo {
            name: subcmd.get_name().to_string(),
            description: subcmd
                .get_about()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            usage,
            arguments,
            flags,
        });
    }

    commands.sort_by(|a, b| a.name.cmp(&b.name));

    let command_count = commands.len();

    let output = CliOutput {
        name,
        version,
        description,
        commands,
    };

    let json = serde_json::to_string_pretty(&output)?;

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output_path = PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("assets/cli.json");

    fs::write(&output_path, &json)?;

    println!("✓ Generated cli.json at: {}", output_path.display());
    println!("  Commands: {}", command_count);

    Ok(())
}
