use super::types::PreviousAiIssue;
use super::AiExecutor;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

impl AiExecutor {
    pub(super) fn format_previous_issues_section(
        &self,
        rule_name: &str,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
        previous_issues: &[PreviousAiIssue],
    ) -> String {
        if previous_issues.is_empty() {
            return String::new();
        }

        let scoped_files: HashSet<String> = files
            .iter()
            .map(|(path, _)| {
                path.strip_prefix(workspace_root)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        let items: Vec<String> = previous_issues
            .iter()
            .filter(|issue| issue.rule == rule_name)
            .filter(|issue| self.previous_issue_in_scope(issue, &scoped_files))
            .map(|issue| self.format_previous_issue(issue))
            .collect();

        if items.is_empty() {
            return String::new();
        }

        format!(
            "## Previous Findings For This Rule\n\nIn addition to the normal rule scan, another AI scan previously reported these findings for this same rule. Re-check each one against the current code. Include a previous finding in the JSON output only if it still exists. Omit findings that are fixed, stale, outside scope, or cannot be verified. Continue reporting any new findings required by the rule.\n\n{}",
            items.join("\n")
        )
    }

    fn previous_issue_in_scope(
        &self,
        issue: &PreviousAiIssue,
        scoped_files: &HashSet<String>,
    ) -> bool {
        let file = issue.file.to_string_lossy().to_string();
        scoped_files.contains(&file) || scoped_files.contains(&file.replace('\\', "/"))
    }

    fn format_previous_issue(&self, issue: &PreviousAiIssue) -> String {
        let mut item = format!(
            "- file: {}\n  line: {}\n  column: {}\n  message: {}",
            issue.file.display(),
            issue.line,
            issue.column,
            issue.message
        );
        if let Some(line_text) = issue.line_text.as_ref().filter(|text| !text.is_empty()) {
            item.push_str(&format!("\n  line_text: `{}`", line_text));
        }
        item
    }
}
