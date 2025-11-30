use super::types::RuleInfo;
use colored::*;
use core::Severity;

pub fn print_header(config_file_path: &str) {
    println!(
        "{}",
        format!("{} Rules Configuration", core::app_display_name())
            .cyan()
            .bold()
    );
    println!("Config: {}\n", config_file_path.dimmed());
}

pub fn print_rules(rules: &[RuleInfo]) {
    let mut enabled_rules: Vec<_> = rules.iter().filter(|r| r.enabled).collect();
    enabled_rules.sort_by(|a, b| a.name.cmp(&b.name));

    if enabled_rules.is_empty() {
        println!("{}", "No rules enabled.".yellow());
        return;
    }

    println!("{} enabled rules:\n", enabled_rules.len());

    for rule in enabled_rules {
        print_rule(rule);
    }

    let disabled_count = rules.iter().filter(|r| !r.enabled).count();
    if disabled_count > 0 {
        println!("\n{} disabled rules", disabled_count.to_string().dimmed());
    }
}

fn print_rule(rule: &RuleInfo) {
    let severity_badge = match rule.severity {
        Severity::Error => "ERROR".red(),
        Severity::Warning => "WARN".yellow(),
    };

    let rule_type = if rule.is_custom() {
        "REGEX".magenta()
    } else {
        "AST".cyan()
    };

    print!("  {} ", "•".cyan());
    print!("{} ", rule.name.bold());
    print!("[{}] ", rule_type);
    println!("{}", severity_badge);

    if let Some(ref pattern) = rule.pattern {
        println!("    Pattern: {}", pattern.yellow());
    }
}
