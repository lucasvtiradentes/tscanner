use super::types::AiError;
use super::{AiExecutor, ChangedLinesMap};
use crate::ai_providers::{parse_provider_error, resolve_provider_command};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tscanner_config::{AiConfig, AiMode, AiRuleConfig};
use tscanner_constants::{
    ai_placeholder_content, ai_placeholder_files, ai_placeholder_options, ai_temp_dir,
};
use tscanner_types::Issue;

const AI_RULE_WRAPPER: &str =
    include_str!("../../../../../../../assets/prompts/ai-rule-wrapper.prompt.md");

impl AiExecutor {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn call_ai_and_parse(
        &self,
        rule_name: &str,
        rule_config: &AiRuleConfig,
        prompt_content: &str,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
        ai_config: &AiConfig,
        changed_lines: Option<&ChangedLinesMap>,
        cancelled: &Arc<AtomicBool>,
    ) -> Result<Vec<Issue>, AiError> {
        let files_section =
            self.format_files_section(files, workspace_root, &rule_config.mode, changed_lines);
        let options_section = if rule_config.options.is_null() {
            String::new()
        } else {
            format!(
                "```json\n{}\n```",
                serde_json::to_string_pretty(&rule_config.options).unwrap_or_default()
            )
        };
        let rule_prompt = prompt_content
            .replace(ai_placeholder_files(), &files_section)
            .replace(ai_placeholder_options(), &options_section);
        let full_prompt = AI_RULE_WRAPPER.replace(ai_placeholder_content(), &rule_prompt);

        self.save_prompt_to_tmp(rule_name, &full_prompt);

        let timeout_secs = rule_config.timeout;
        let timeout_ms = if timeout_secs > 0 {
            timeout_secs * 1000
        } else {
            0
        };
        let (program, args) =
            resolve_provider_command(ai_config.provider.as_ref(), ai_config.command.as_deref())
                .map_err(AiError::InvalidOutput)?;

        let mode_str = match rule_config.mode {
            AiMode::Paths => "paths",
            AiMode::Content => "content",
            AiMode::Agentic => "agentic",
        };

        (self.log_warn)(&format!(
            "AI rule '{}': calling {} with {} files, mode={} (timeout: {}s)",
            rule_name,
            program,
            files.len(),
            mode_str,
            timeout_secs
        ));

        let response = self
            .spawn_ai_command(&program, &args, &full_prompt, timeout_ms, cancelled)
            .map_err(|e| match e {
                AiError::NonZeroExit { code, stderr } => {
                    let friendly = parse_provider_error(ai_config.provider.as_ref(), &stderr);
                    AiError::NonZeroExit {
                        code,
                        stderr: friendly,
                    }
                }
                other => other,
            })?;

        if cancelled.load(Ordering::SeqCst) {
            return Ok(vec![]);
        }

        self.parse_response(rule_name, rule_config, &response, workspace_root, files)
    }

    fn format_files_section(
        &self,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
        mode: &AiMode,
        changed_lines: Option<&ChangedLinesMap>,
    ) -> String {
        match mode {
            AiMode::Paths => {
                let paths: Vec<_> = files
                    .iter()
                    .map(|(path, _)| {
                        let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                        let line_info =
                            self.format_changed_lines_info(path, workspace_root, changed_lines);
                        if line_info.is_empty() {
                            format!("- {}", relative.display())
                        } else {
                            format!("- {} {}", relative.display(), line_info)
                        }
                    })
                    .collect();
                format!(
                    "## Files to analyze\n\n{}\n\nRead each file and analyze according to the rules above.",
                    paths.join("\n")
                )
            }
            AiMode::Content => files
                .iter()
                .map(|(path, content)| {
                    let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                    let line_info =
                        self.format_changed_lines_info(path, workspace_root, changed_lines);
                    if line_info.is_empty() {
                        format!("### File: {}\n```\n{}\n```\n", relative.display(), content)
                    } else {
                        format!(
                            "### File: {} {}\n```\n{}\n```\n",
                            relative.display(),
                            line_info,
                            content
                        )
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
            AiMode::Agentic => {
                let paths: Vec<_> = files
                    .iter()
                    .map(|(path, _)| {
                        let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                        let line_info =
                            self.format_changed_lines_info(path, workspace_root, changed_lines);
                        if line_info.is_empty() {
                            format!("- {}", relative.display())
                        } else {
                            format!("- {} {}", relative.display(), line_info)
                        }
                    })
                    .collect();
                format!(
                    "## Scope\n\nYou have access to explore the codebase freely. Start by investigating these files:\n\n{}\n\nYou may read additional files as needed to complete the analysis.",
                    paths.join("\n")
                )
            }
        }
    }

    fn format_changed_lines_info(
        &self,
        path: &Path,
        workspace_root: &Path,
        changed_lines: Option<&ChangedLinesMap>,
    ) -> String {
        let Some(lines_map) = changed_lines else {
            return String::new();
        };

        if let Some(lines) = lines_map.get(path) {
            if !lines.is_empty() {
                return self.format_line_ranges(lines);
            }
        }

        let relative = path.strip_prefix(workspace_root).unwrap_or(path);
        let Some(lines) = lines_map.get(relative) else {
            return String::new();
        };

        if lines.is_empty() {
            return String::new();
        }

        self.format_line_ranges(lines)
    }

    fn format_line_ranges(&self, lines: &HashSet<usize>) -> String {
        let ranges = self.lines_to_ranges(lines);
        let range_strs: Vec<String> = ranges
            .iter()
            .map(|(start, end)| {
                if start == end {
                    format!("{}", start)
                } else {
                    format!("{}-{}", start, end)
                }
            })
            .collect();

        format!("(modified lines: {})", range_strs.join(", "))
    }

    fn lines_to_ranges(&self, lines: &HashSet<usize>) -> Vec<(usize, usize)> {
        let mut sorted: Vec<usize> = lines.iter().copied().collect();
        sorted.sort();

        if sorted.is_empty() {
            return vec![];
        }

        let mut ranges = Vec::new();
        let mut start = sorted[0];
        let mut end = sorted[0];

        for &line in sorted.iter().skip(1) {
            if line == end + 1 {
                end = line;
            } else {
                ranges.push((start, end));
                start = line;
                end = line;
            }
        }
        ranges.push((start, end));

        ranges
    }

    pub(super) fn save_prompt_to_tmp(&self, rule_name: &str, prompt: &str) {
        let tmp_dir = std::env::temp_dir().join(ai_temp_dir());
        if std::fs::create_dir_all(&tmp_dir).is_err() {
            return;
        }

        let safe_name = rule_name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        let filename = format!("{}.md", safe_name);
        let filepath = tmp_dir.join(&filename);

        let _ = std::fs::write(&filepath, prompt);
    }
}
