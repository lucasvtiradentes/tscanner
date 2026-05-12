use super::types::{AiError, AiResponse};
use super::AiExecutor;
use crate::executors::utils;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tscanner_config::AiRuleConfig;
use tscanner_types::{Issue, IssueRuleType};

impl AiExecutor {
    pub(super) fn parse_response(
        &self,
        rule_name: &str,
        rule_config: &AiRuleConfig,
        response: &str,
        workspace_root: &Path,
        files: &[&(PathBuf, String)],
    ) -> Result<Vec<Issue>, AiError> {
        let json_start = response.find('{');
        let json_end = response.rfind('}');

        let json_str = match (json_start, json_end) {
            (Some(start), Some(end)) if end >= start => &response[start..=end],
            _ => {
                if response.trim().is_empty() {
                    return Ok(vec![]);
                }
                (self.log_debug)(&format!(
                    "AI rule '{}': no JSON found in response ({}chars)",
                    rule_name,
                    response.len()
                ));
                return Ok(vec![]);
            }
        };

        let ai_response: AiResponse = serde_json::from_str(json_str).map_err(|e| {
            AiError::InvalidOutput(format!(
                "Failed to parse JSON: {} - Output: {}",
                e,
                json_str.chars().take(500).collect::<String>()
            ))
        })?;

        (self.log_warn)(&format!(
            "AI rule '{}': parsed {} raw issues from response",
            rule_name,
            ai_response.issues.len()
        ));

        let file_lines: HashMap<PathBuf, Vec<&str>> = files
            .iter()
            .map(|(path, content)| {
                let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                (relative.to_path_buf(), content.lines().collect())
            })
            .collect();

        if !ai_response.issues.is_empty() {
            let known_files: Vec<_> = file_lines.keys().map(|p| p.display().to_string()).collect();
            (self.log_warn)(&format!(
                "AI rule '{}': known files ({}): {:?}",
                rule_name,
                known_files.len(),
                known_files.iter().take(5).collect::<Vec<_>>()
            ));
        }

        let issues: Vec<_> = ai_response
            .issues
            .into_iter()
            .filter_map(|issue| {
                let file_path = PathBuf::from(&issue.file);

                if let Some(lines) = file_lines.get(&file_path) {
                    if issue.line == 0 || issue.line > lines.len() {
                        (self.log_warn)(&format!(
                            "AI returned invalid line {} for file {} (max: {})",
                            issue.line,
                            issue.file,
                            lines.len()
                        ));
                        return None;
                    }
                } else {
                    (self.log_warn)(&format!(
                        "AI returned unknown file: {} (not in input files: {:?})",
                        issue.file,
                        file_lines.keys().take(3).collect::<Vec<_>>()
                    ));
                    return None;
                }

                let line_text = file_lines
                    .get(&file_path)
                    .and_then(|lines| utils::extract_line_text(lines, issue.line));

                Some(Issue {
                    rule: rule_name.to_string(),
                    file: workspace_root.join(&issue.file),
                    line: issue.line,
                    column: issue.column.max(1),
                    end_column: issue.column.max(1) + 1,
                    message: issue.message,
                    severity: rule_config.severity,
                    line_text,
                    category: None,
                    rule_type: IssueRuleType::Ai,
                })
            })
            .collect();

        (self.log_warn)(&format!(
            "AI rule '{}': {} issues after validation",
            rule_name,
            issues.len()
        ));

        Ok(issues)
    }

    pub(super) fn validate_cached_issues(
        &self,
        cached_issues: &[Issue],
        current_files: &[&(PathBuf, String)],
        workspace_root: &Path,
    ) -> Vec<Issue> {
        cached_issues
            .iter()
            .filter(|issue| {
                let relative_path = issue
                    .file
                    .strip_prefix(workspace_root)
                    .unwrap_or(&issue.file);

                if let Some((_, content)) = current_files
                    .iter()
                    .find(|(p, _)| p.strip_prefix(workspace_root).unwrap_or(p) == relative_path)
                {
                    let lines: Vec<&str> = content.lines().collect();
                    issue.line > 0 && issue.line <= lines.len()
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }
}
