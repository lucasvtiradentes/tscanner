use anyhow::{Context, Result};
use colored::*;
use dialoguer::MultiSelect;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::shared::{fatal_error_and_exit, print_section_title};
use tscanner_cli::RegistryRuleKind;
use tscanner_constants::{config_dir_name, config_file_name, registry_base_url_for_ref};
use tscanner_service::log_info;

const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

use super::config_updater::update_config_with_rule;
use super::fetcher::{
    fetch_registry_index, fetch_rule_config, fetch_rule_file, filter_rules, RegistryRule,
};
use super::installer::{get_rule_local_path, install_rule_file};

pub fn cmd_registry(
    name: Option<String>,
    kind: Option<RegistryRuleKind>,
    category: Option<String>,
    force: bool,
    latest: bool,
    config_path: Option<PathBuf>,
) -> Result<()> {
    log_info(&format!(
        "cmd_registry: name={:?}, kind={:?}, category={:?}, force={}, latest={}, config_path={:?}",
        name, kind, category, force, latest, config_path
    ));

    let workspace_root = std::env::current_dir().context("Failed to get current directory")?;
    let config_dir = resolve_config_dir(&workspace_root, config_path.clone());
    let config_file = config_dir.join(config_file_name());

    if !config_file.exists() {
        fatal_error_and_exit(
            "No TScanner configuration found",
            &[
                &format!("Expected config at: {}", config_file.display()),
                "",
                "Run 'tscanner init' first to create a configuration file.",
            ],
        );
    }

    let git_ref = if latest {
        "main".to_string()
    } else {
        format!("v{}", CLI_VERSION)
    };
    let registry_base_url = registry_base_url_for_ref(&git_ref);

    println!(
        "{}",
        format!("Fetching registry [{}]...", registry_base_url).dimmed()
    );

    let index = match fetch_registry_index(&registry_base_url) {
        Ok(idx) => idx,
        Err(e) => {
            fatal_error_and_exit(
                "Failed to fetch registry",
                &[
                    &format!("{}", e),
                    "",
                    "Check your internet connection and try again.",
                ],
            );
        }
    };

    let kind_str = kind.as_ref().map(|k| k.as_str());
    let category_str = category.as_deref();
    let filtered_rules = filter_rules(index.rules, kind_str, category_str);

    if filtered_rules.is_empty() {
        println!();
        println!(
            "{}",
            "No rules found matching the specified filters.".yellow()
        );
        return Ok(());
    }

    let installed_rules = get_installed_rules(&config_file);

    let rules_to_install = if let Some(ref rule_name) = name {
        let rule = filtered_rules
            .iter()
            .find(|r| r.name == *rule_name)
            .cloned();

        match rule {
            Some(r) => vec![r],
            None => {
                fatal_error_and_exit(
                    &format!("Rule '{}' not found", rule_name),
                    &[
                        "",
                        "Use 'tscanner registry' without arguments to see available rules.",
                    ],
                );
            }
        }
    } else {
        select_rules_interactive(&filtered_rules, &installed_rules)?
    };

    if rules_to_install.is_empty() {
        println!("{}", "No rules selected.".dimmed());
        return Ok(());
    }

    let effective_root = config_path
        .map(|p| {
            if p.is_absolute() {
                p
            } else {
                workspace_root.join(p)
            }
        })
        .unwrap_or(workspace_root);

    install_rules(
        &effective_root,
        &rules_to_install,
        force,
        &registry_base_url,
    )?;

    Ok(())
}

fn resolve_config_dir(root: &Path, config_path: Option<PathBuf>) -> PathBuf {
    match config_path {
        Some(dir) => {
            if dir.is_absolute() {
                dir.join(config_dir_name())
            } else {
                root.join(dir).join(config_dir_name())
            }
        }
        None => root.join(config_dir_name()),
    }
}

fn get_installed_rules(config_file: &Path) -> Vec<String> {
    let content = match fs::read_to_string(config_file) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let stripped = json_comments::StripComments::new(content.as_bytes());
    let json: Value = match serde_json::from_reader(stripped) {
        Ok(j) => j,
        Err(_) => return vec![],
    };

    let mut installed = vec![];

    if let Some(ai_rules) = json.get("aiRules").and_then(|v| v.as_object()) {
        for key in ai_rules.keys() {
            installed.push(key.clone());
        }
    }

    if let Some(rules) = json.get("rules").and_then(|v| v.as_object()) {
        if let Some(script) = rules.get("script").and_then(|v| v.as_object()) {
            for key in script.keys() {
                installed.push(key.clone());
            }
        }
        if let Some(regex) = rules.get("regex").and_then(|v| v.as_object()) {
            for key in regex.keys() {
                installed.push(key.clone());
            }
        }
    }

    installed
}

fn get_file_extension(rule: &RegistryRule) -> String {
    match &rule.file {
        Some(f) => {
            if let Some(ext) = Path::new(f).extension() {
                format!(".{}", ext.to_string_lossy())
            } else {
                "-".to_string()
            }
        }
        None => match rule.kind.as_str() {
            "ai" => ".md".to_string(),
            "script" => ".ts".to_string(),
            "regex" => "-".to_string(),
            _ => "-".to_string(),
        },
    }
}

fn select_rules_interactive(
    rules: &[RegistryRule],
    installed_rules: &[String],
) -> Result<Vec<RegistryRule>> {
    let max_kind_len = rules.iter().map(|r| r.kind.len()).max().unwrap_or(6);
    let max_name_len = rules.iter().map(|r| r.name.len()).max().unwrap_or(20);

    let items: Vec<String> = rules
        .iter()
        .map(|r| {
            let is_installed = installed_rules.contains(&r.name);
            let ext = get_file_extension(r);

            let base = format!(
                "{:<width_kind$} | {:<4} | {:<width_name$} - {}",
                r.kind,
                ext,
                r.name,
                r.description,
                width_kind = max_kind_len,
                width_name = max_name_len
            );

            if is_installed {
                format!("{}", base.dimmed())
            } else {
                base
            }
        })
        .collect();

    println!();
    println!("Select rules to install (Space to select, Enter to confirm):");
    println!();

    let selections = MultiSelect::new()
        .items(&items)
        .interact()
        .context("Failed to get user selection")?;

    Ok(selections.into_iter().map(|i| rules[i].clone()).collect())
}

fn install_rules(
    workspace_root: &Path,
    rules: &[RegistryRule],
    force: bool,
    registry_base_url: &str,
) -> Result<()> {
    println!();
    print_section_title("Installing rules:");
    println!();

    let mut installed: Vec<RegistryRule> = Vec::new();
    let mut errors = Vec::new();

    for rule in rules {
        print!("  {} {}...", "→".blue(), rule.name);

        match install_single_rule(workspace_root, rule, force, registry_base_url) {
            Ok(()) => {
                println!(" {}", "✓".green());
                installed.push(rule.clone());
            }
            Err(e) => {
                println!(" {}", "✗".red());
                errors.push((rule.name.clone(), e.to_string()));
            }
        }
    }

    println!();

    if !installed.is_empty() {
        println!(
            "{}",
            format!("✓ Installed {} rule(s)", installed.len())
                .green()
                .bold()
        );
        for rule in &installed {
            let local_path = get_rule_local_path(rule);
            if !local_path.is_empty() {
                println!("  {} {}", "→".dimmed(), local_path);
            }
            println!(
                "  {} config.jsonc updated with '{}'",
                "→".dimmed(),
                rule.name
            );
        }
    }

    if !errors.is_empty() {
        println!();
        println!(
            "{}",
            format!("✗ Failed to install {} rule(s)", errors.len())
                .red()
                .bold()
        );
        for (name, error) in &errors {
            println!("  {} {}: {}", "→".red(), name, error);
        }
    }

    Ok(())
}

fn install_single_rule(
    workspace_root: &Path,
    rule: &RegistryRule,
    force: bool,
    registry_base_url: &str,
) -> Result<()> {
    let config =
        fetch_rule_config(registry_base_url, rule).context("Failed to fetch rule config")?;

    if let Some((filename, content)) =
        fetch_rule_file(registry_base_url, rule).context("Failed to fetch rule file")?
    {
        install_rule_file(workspace_root, rule, &filename, &content, force)?;
    }

    update_config_with_rule(workspace_root, rule, &config, force)?;

    Ok(())
}
