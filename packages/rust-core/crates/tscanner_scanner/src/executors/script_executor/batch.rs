use super::{ScriptError, ScriptExecutor, ScriptFile, ScriptInput, ScriptOutput};
use crate::executors::utils;
use std::collections::HashMap;
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
        let script_files: Vec<ScriptFile> = files
            .iter()
            .map(|(path, content)| {
                let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                ScriptFile {
                    path: relative.to_string_lossy().to_string(),
                    content: content.clone(),
                    lines: content.lines().map(String::from).collect(),
                }
            })
            .collect();

        let options = if rule_config.options.is_null() {
            None
        } else {
            Some(rule_config.options.clone())
        };

        let input = ScriptInput {
            files: script_files,
            options,
            workspace_root: workspace_root.to_string_lossy().to_string(),
        };

        let input_json = serde_json::to_vec(&input)
            .map_err(|e| ScriptError::InvalidOutput(format!("Failed to serialize input: {}", e)))?;

        let output = self.spawn_command(rule_config, &input_json)?;

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

        Ok(script_output
            .issues
            .into_iter()
            .map(|issue| {
                let file_path = workspace_root.join(&issue.file);
                let relative_path = PathBuf::from(&issue.file);
                let line_text = file_lines
                    .get(&relative_path)
                    .and_then(|lines| utils::extract_line_text(lines, issue.line));
                Issue {
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
                }
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
}
