use super::{ScriptError, ScriptExecutor, ScriptOutput};
use crate::executors::utils;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tscanner_config::ScriptRuleConfig;
use tscanner_types::{Issue, IssueRuleType};

impl ScriptExecutor {
    pub(super) fn execute_batch(
        &self,
        rule_name: &str,
        rule_config: &ScriptRuleConfig,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
    ) -> Result<Vec<Issue>, ScriptError> {
        let script_files: Vec<String> = files
            .iter()
            .map(|(path, _)| {
                let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                relative.to_string_lossy().to_string()
            })
            .collect();

        let output = self.spawn_command(rule_config, workspace_root, &script_files)?;

        self.parse_output(rule_name, rule_config, &output, workspace_root, files)
    }

    fn parse_output(
        &self,
        rule_name: &str,
        rule_config: &ScriptRuleConfig,
        output: &[u8],
        workspace_root: &Path,
        files: &[&(PathBuf, String)],
    ) -> Result<Vec<Issue>, ScriptError> {
        let output_str = String::from_utf8_lossy(output);
        let json_str = self.extract_json(&output_str)?;
        let script_output: ScriptOutput = serde_json::from_str(json_str).map_err(|e| {
            ScriptError::InvalidOutput(format!(
                "Failed to parse JSON: {} - Output: {}",
                e,
                json_str.chars().take(500).collect::<String>()
            ))
        })?;
        let file_lines = self.collect_file_lines(workspace_root, files);
        let allowed_files: HashSet<PathBuf> = file_lines.keys().cloned().collect();

        Ok(script_output
            .issues
            .into_iter()
            .filter_map(|issue| {
                let relative_path = self.normalize_issue_path(&issue.file, workspace_root);
                if !allowed_files.contains(&relative_path) {
                    return None;
                }
                let file_path = workspace_root.join(&relative_path);
                let line_text = file_lines
                    .get(&relative_path)
                    .and_then(|lines| utils::extract_line_text(lines, issue.line));
                Some(Issue {
                    rule: rule_name.to_string(),
                    file: file_path,
                    line: issue.line,
                    column: if issue.column > 0 { issue.column } else { 1 },
                    end_column: if issue.column > 0 {
                        issue.column + 1
                    } else {
                        1
                    },
                    message: issue.message,
                    severity: rule_config.severity,
                    line_text,
                    category: None,
                    rule_type: IssueRuleType::CustomScript,
                })
            })
            .collect())
    }

    fn extract_json<'a>(&self, output: &'a str) -> Result<&'a str, ScriptError> {
        match output.find('{') {
            Some(start) => Ok(&output[start..]),
            None if output.trim().is_empty() => Ok("{\"issues\":[]}"),
            None => Err(ScriptError::InvalidOutput(format!(
                "No JSON found in output: {}",
                output.chars().take(200).collect::<String>()
            ))),
        }
    }

    fn collect_file_lines<'a>(
        &self,
        workspace_root: &Path,
        files: &'a [&(PathBuf, String)],
    ) -> HashMap<PathBuf, Vec<&'a str>> {
        files
            .iter()
            .map(|(path, content)| {
                let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                (relative.to_path_buf(), content.lines().collect())
            })
            .collect()
    }

    fn normalize_issue_path(&self, file: &str, workspace_root: &Path) -> PathBuf {
        let path = PathBuf::from(file);
        if path.is_absolute() {
            path.strip_prefix(workspace_root)
                .unwrap_or(&path)
                .to_path_buf()
        } else {
            path
        }
    }
}
