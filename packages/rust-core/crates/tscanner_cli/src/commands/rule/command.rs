use anyhow::{Context, Result};
use colored::*;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};
use dialoguer::MultiSelect;
use std::path::{Path, PathBuf};

use crate::shared::{fatal_error_and_exit, print_section_title};
use tscanner_cli::RegistryRuleKind;
use tscanner_constants::{config_dir_name, config_file_name};
use tscanner_service::log_info;

use super::config_updater::update_config_with_rule;
use super::installer::{get_rule_local_path, install_rule_file};
use super::registry::{
    fetch_registry_index, fetch_rule_config, fetch_rule_file, filter_rules, RegistryRule,
};

pub fn cmd_rule(
    name: Option<String>,
    kind: Option<RegistryRuleKind>,
    category: Option<String>,
    force: bool,
    config_path: Option<PathBuf>,
) -> Result<()> {
    log_info(&format!(
        "cmd_rule: name={:?}, kind={:?}, category={:?}, force={}, config_path={:?}",
        name, kind, category, force, config_path
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

    println!("{}", "Fetching registry...".dimmed());

    let index = match fetch_registry_index() {
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
                        "Use 'tscanner rule' without arguments to see available rules.",
                    ],
                );
            }
        }
    } else {
        select_rules_interactive(&filtered_rules)?
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

    install_rules(&effective_root, &rules_to_install, force)?;

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

fn select_rules_interactive(rules: &[RegistryRule]) -> Result<Vec<RegistryRule>> {
    println!();
    print_rules_table(rules);
    println!();

    let items: Vec<String> = rules
        .iter()
        .map(|r| format!("[{}] {} - {}", r.kind, r.name, r.description))
        .collect();

    let selections = MultiSelect::new()
        .with_prompt("Select rules to install (Space to select, Enter to confirm)")
        .items(&items)
        .interact()
        .context("Failed to get user selection")?;

    Ok(selections.into_iter().map(|i| rules[i].clone()).collect())
}

fn print_rules_table(rules: &[RegistryRule]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Kind").fg(Color::Cyan),
            Cell::new("Name").fg(Color::Cyan),
            Cell::new("Category").fg(Color::Cyan),
            Cell::new("Description").fg(Color::Cyan),
        ]);

    for rule in rules {
        let kind_color = match rule.kind.as_str() {
            "ai" => Color::Magenta,
            "script" => Color::Blue,
            "regex" => Color::Yellow,
            _ => Color::White,
        };

        table.add_row(vec![
            Cell::new(&rule.kind).fg(kind_color),
            Cell::new(&rule.name),
            Cell::new(&rule.category),
            Cell::new(&rule.description),
        ]);
    }

    println!("{table}");
}

fn install_rules(workspace_root: &Path, rules: &[RegistryRule], force: bool) -> Result<()> {
    println!();
    print_section_title("Installing rules:");
    println!();

    let mut installed: Vec<RegistryRule> = Vec::new();
    let mut errors = Vec::new();

    for rule in rules {
        print!("  {} {}...", "→".blue(), rule.name);

        match install_single_rule(workspace_root, rule, force) {
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

fn install_single_rule(workspace_root: &Path, rule: &RegistryRule, force: bool) -> Result<()> {
    let config = fetch_rule_config(rule).context("Failed to fetch rule config")?;

    if let Some((filename, content)) = fetch_rule_file(rule).context("Failed to fetch rule file")? {
        install_rule_file(workspace_root, rule, &filename, &content, force)?;
    }

    update_config_with_rule(workspace_root, rule, &config, force)?;

    Ok(())
}
