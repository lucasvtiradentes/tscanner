use crate::shared::severity_icon;
use colored::*;

fn get_severity_icon(severity: &str) -> ColoredString {
    let icon = severity_icon(severity);
    match severity {
        "error" => icon.red(),
        "warning" => icon.yellow(),
        "info" => icon.blue(),
        "hint" => icon.dimmed(),
        _ => icon.yellow(),
    }
}

pub(super) trait IssueDisplay {
    fn severity(&self) -> &str;
    fn line(&self) -> usize;
    fn column(&self) -> usize;
    fn line_text(&self) -> Option<&str>;
}

impl IssueDisplay for tscanner_cli_output::OutputIssue {
    fn severity(&self) -> &str {
        &self.severity
    }
    fn line(&self) -> usize {
        self.line
    }
    fn column(&self) -> usize {
        self.column
    }
    fn line_text(&self) -> Option<&str> {
        self.line_text.as_deref()
    }
}

impl IssueDisplay for tscanner_cli_output::OutputRuleIssue {
    fn severity(&self) -> &str {
        &self.severity
    }
    fn line(&self) -> usize {
        self.line
    }
    fn column(&self) -> usize {
        self.column
    }
    fn line_text(&self) -> Option<&str> {
        self.line_text.as_deref()
    }
}

pub(super) fn render_issue_location<T: IssueDisplay>(issue: &T) {
    let severity_icon = get_severity_icon(issue.severity());
    let location = format!("{}:{}", issue.line(), issue.column());

    if let Some(line_text) = issue.line_text() {
        let trimmed = line_text.trim();
        if !trimmed.is_empty() {
            println!(
                "    {} {} → {}",
                severity_icon,
                location.dimmed(),
                trimmed.dimmed()
            );
            return;
        }
    }
    println!("    {} {}", severity_icon, location.dimmed());
}
