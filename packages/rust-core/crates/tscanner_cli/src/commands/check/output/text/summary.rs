use crate::shared::{format_duration, print_section_header, OutputSummary};
use colored::*;
use tscanner_constants::{
    icon_ai, icon_builtin, icon_error, icon_hint, icon_info, icon_regex, icon_script, icon_warning,
};

fn format_rule_breakdown(parts: &[(usize, &'static str)]) -> String {
    if parts.is_empty() {
        return String::new();
    }
    let formatted: Vec<String> = parts
        .iter()
        .map(|(count, label)| {
            let icon = match *label {
                "builtin" => icon_builtin(),
                "regex" => icon_regex(),
                "script" => icon_script(),
                "ai" => icon_ai(),
                _ => icon_builtin(),
            };
            format!("{} {}", icon, count)
        })
        .collect();
    format!(" ({})", formatted.join(", "))
}

pub fn render_summary(summary: &OutputSummary) {
    print_section_header("Scope:");

    let enabled_breakdown_str = format_rule_breakdown(&summary.enabled_rules_breakdown_parts());

    println!(
        "  {} {}{}",
        "Rules:".dimmed(),
        summary.total_enabled_rules.to_string().cyan(),
        enabled_breakdown_str
    );

    println!(
        "  {} {} ({} cached, {} scanned)",
        "Files:".dimmed(),
        summary.total_files.to_string().cyan(),
        summary.cached_files,
        summary.scanned_files
    );

    println!();
    print_section_header("Results:");

    let issue_parts = summary.issue_parts();
    if issue_parts.is_empty() {
        println!(
            "  {} {}",
            "Issues:".dimmed(),
            summary.total_issues.to_string().cyan(),
        );
    } else {
        let colored_parts: Vec<String> = issue_parts
            .iter()
            .map(|p| {
                let (icon, colored_count) = match p.label {
                    "errors" => (icon_error(), p.count.to_string().red().to_string()),
                    "warnings" => (icon_warning(), p.count.to_string().yellow().to_string()),
                    "infos" => (icon_info(), p.count.to_string().blue().to_string()),
                    "hints" => (icon_hint(), p.count.to_string().dimmed().to_string()),
                    _ => (icon_warning(), p.count.to_string()),
                };
                format!("{} {}", icon, colored_count)
            })
            .collect();
        println!(
            "  {} {} ({})",
            "Issues:".dimmed(),
            summary.total_issues.to_string().cyan(),
            colored_parts.join(", ")
        );
    }

    let breakdown_str = format_rule_breakdown(&summary.rules_breakdown_parts());

    println!(
        "  {} {}{}",
        "Triggered rules:".dimmed(),
        summary.triggered_rules.to_string().cyan(),
        breakdown_str
    );

    println!(
        "  {} {}",
        "Files with issues:".dimmed(),
        summary.files_with_issues.to_string().cyan()
    );

    println!(
        "  {} {}",
        "Duration:".dimmed(),
        format_duration(summary.duration_ms)
    );

    println!();
}
