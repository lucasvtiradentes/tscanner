use crate::ai_providers::resolve_provider_command;
use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tscanner_cache::AiCache;
use tscanner_config::{AiConfig, AiMode, AiRuleConfig};
use tscanner_constants::{
    ai_placeholder_content, ai_placeholder_files, ai_placeholder_options, ai_rules_dir,
    ai_temp_dir, config_dir_name,
};
use tscanner_types::{Issue, IssueRuleType};

pub type ChangedLinesMap = HashMap<PathBuf, HashSet<usize>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRuleStatus {
    Pending {},
    Running {},
    Completed { issues_found: usize },
    Failed { error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProgressEvent {
    pub rule_name: String,
    pub rule_index: usize,
    pub total_rules: usize,
    pub status: AiRuleStatus,
}

pub type AiProgressCallback = Arc<dyn Fn(AiProgressEvent) + Send + Sync>;
pub type RegularRulesCompleteCallback = Arc<dyn Fn(u128) + Send + Sync>;

const AI_RULE_WRAPPER: &str =
    include_str!("../../../../../../assets/prompts/ai-rule-wrapper.prompt.md");

#[derive(Debug, Deserialize)]
pub struct AiResponse {
    pub issues: Vec<AiIssue>,
}

#[derive(Debug, Deserialize)]
pub struct AiIssue {
    pub file: String,
    pub line: usize,
    #[serde(default)]
    pub column: usize,
    pub message: String,
}

#[derive(Debug)]
pub enum AiError {
    IoError(std::io::Error),
    Timeout(u64),
    NonZeroExit { code: Option<i32>, stderr: String },
    InvalidOutput(String),
    ProviderNotFound(String),
    PromptNotFound(PathBuf),
}

impl std::fmt::Display for AiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiError::IoError(e) => write!(f, "IO error: {}", e),
            AiError::Timeout(secs) => write!(f, "AI call timed out after {}s", secs),
            AiError::NonZeroExit { code, stderr } => {
                write!(f, "AI command exited with code {:?}: {}", code, stderr)
            }
            AiError::InvalidOutput(msg) => write!(f, "Invalid AI output: {}", msg),
            AiError::ProviderNotFound(cmd) => {
                write!(
                    f,
                    "AI provider '{}' not found. Install it or check PATH",
                    cmd
                )
            }
            AiError::PromptNotFound(path) => write!(f, "Prompt file not found: {:?}", path),
        }
    }
}

impl From<std::io::Error> for AiError {
    fn from(e: std::io::Error) -> Self {
        AiError::IoError(e)
    }
}

pub struct AiExecutor {
    workspace_root: PathBuf,
    ai_rules_dir: PathBuf,
    ai_config: Option<AiConfig>,
    cache: Arc<AiCache>,
    in_flight: DashMap<String, Arc<AtomicBool>>,
    log_warn: fn(&str),
    log_debug: fn(&str),
}

impl AiExecutor {
    pub fn new(
        workspace_root: &Path,
        config_dir: Option<PathBuf>,
        ai_config: Option<AiConfig>,
        cache: Arc<AiCache>,
        log_warn: Option<fn(&str)>,
        log_debug: Option<fn(&str)>,
    ) -> Self {
        let ai_rules_dir_path = config_dir
            .map(|d| d.join(ai_rules_dir()))
            .unwrap_or_else(|| workspace_root.join(config_dir_name()).join(ai_rules_dir()));
        Self {
            workspace_root: workspace_root.to_path_buf(),
            ai_rules_dir: ai_rules_dir_path,
            ai_config,
            cache,
            in_flight: DashMap::new(),
            log_warn: log_warn.unwrap_or(|_| {}),
            log_debug: log_debug.unwrap_or(|_| {}),
        }
    }

    pub fn with_logger(workspace_root: &Path, log_warn: fn(&str), log_debug: fn(&str)) -> Self {
        Self::new(
            workspace_root,
            None,
            None,
            Arc::new(AiCache::new()),
            Some(log_warn),
            Some(log_debug),
        )
    }

    pub fn with_config(
        workspace_root: &Path,
        ai_config: Option<AiConfig>,
        cache: Arc<AiCache>,
        log_warn: fn(&str),
        log_debug: fn(&str),
    ) -> Self {
        Self::new(
            workspace_root,
            None,
            ai_config,
            cache,
            Some(log_warn),
            Some(log_debug),
        )
    }

    pub fn with_config_dir(
        workspace_root: &Path,
        config_dir: PathBuf,
        ai_config: Option<AiConfig>,
        cache: Arc<AiCache>,
        log_warn: fn(&str),
        log_debug: fn(&str),
    ) -> Self {
        Self::new(
            workspace_root,
            Some(config_dir),
            ai_config,
            cache,
            Some(log_warn),
            Some(log_debug),
        )
    }

    pub fn execute_rules(
        &self,
        rules: &[(String, AiRuleConfig)],
        files: &[(PathBuf, String)],
        workspace_root: &Path,
        changed_lines: Option<&ChangedLinesMap>,
    ) -> (Vec<Issue>, Option<String>) {
        self.execute_rules_with_progress(rules, files, workspace_root, changed_lines, None)
    }

    pub fn execute_rules_with_progress(
        &self,
        rules: &[(String, AiRuleConfig)],
        files: &[(PathBuf, String)],
        workspace_root: &Path,
        changed_lines: Option<&ChangedLinesMap>,
        progress_callback: Option<AiProgressCallback>,
    ) -> (Vec<Issue>, Option<String>) {
        if rules.is_empty() {
            return (vec![], None);
        }

        let ai_config = match &self.ai_config {
            Some(config) => config,
            None => {
                let warning = format!(
                    "AI rules configured ({} rules) but 'ai' config section is missing. Add 'ai.provider' to your config.",
                    rules.len()
                );
                (self.log_warn)(&warning);
                return (vec![], Some(warning));
            }
        };

        let total_rules = rules.len();
        let completed_count = Arc::new(AtomicUsize::new(0));

        if let Some(ref cb) = progress_callback {
            for (idx, (rule_name, _)) in rules.iter().enumerate() {
                cb(AiProgressEvent {
                    rule_name: rule_name.clone(),
                    rule_index: idx,
                    total_rules,
                    status: AiRuleStatus::Pending {},
                });
            }
        }

        let all_issues: Vec<Issue> = rules
            .par_iter()
            .enumerate()
            .flat_map(|(idx, (rule_name, rule_config))| {
                if let Some(ref cb) = progress_callback {
                    cb(AiProgressEvent {
                        rule_name: rule_name.clone(),
                        rule_index: idx,
                        total_rules,
                        status: AiRuleStatus::Running {},
                    });
                }

                let matching_files: Vec<_> = files
                    .iter()
                    .filter(|(path, _)| self.file_matches_rule(path, workspace_root, rule_config))
                    .collect();

                if matching_files.is_empty() {
                    completed_count.fetch_add(1, Ordering::SeqCst);
                    if let Some(ref cb) = progress_callback {
                        cb(AiProgressEvent {
                            rule_name: rule_name.clone(),
                            rule_index: idx,
                            total_rules,
                            status: AiRuleStatus::Completed { issues_found: 0 },
                        });
                    }
                    return vec![];
                }

                let result = self.execute_rule(
                    rule_name,
                    rule_config,
                    &matching_files,
                    workspace_root,
                    ai_config,
                    changed_lines,
                );

                completed_count.fetch_add(1, Ordering::SeqCst);

                match result {
                    Ok(issues) => {
                        if let Some(ref cb) = progress_callback {
                            cb(AiProgressEvent {
                                rule_name: rule_name.clone(),
                                rule_index: idx,
                                total_rules,
                                status: AiRuleStatus::Completed {
                                    issues_found: issues.len(),
                                },
                            });
                        }
                        issues
                    }
                    Err(e) => {
                        (self.log_warn)(&format!("AI rule '{}' failed: {}", rule_name, e));
                        if let Some(ref cb) = progress_callback {
                            cb(AiProgressEvent {
                                rule_name: rule_name.clone(),
                                rule_index: idx,
                                total_rules,
                                status: AiRuleStatus::Failed {
                                    error: e.to_string(),
                                },
                            });
                        }
                        vec![]
                    }
                }
            })
            .collect();

        (all_issues, None)
    }

    fn file_matches_rule(
        &self,
        path: &Path,
        workspace_root: &Path,
        rule_config: &AiRuleConfig,
    ) -> bool {
        super::utils::file_matches_patterns(
            path,
            workspace_root,
            &rule_config.include,
            &rule_config.exclude,
        )
    }

    fn execute_rule(
        &self,
        rule_name: &str,
        rule_config: &AiRuleConfig,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
        ai_config: &AiConfig,
        changed_lines: Option<&ChangedLinesMap>,
    ) -> Result<Vec<Issue>, AiError> {
        let prompt_path = self.ai_rules_dir.join(&rule_config.prompt);
        if !prompt_path.exists() {
            return Err(AiError::PromptNotFound(prompt_path));
        }

        let files_owned: Vec<(PathBuf, String)> =
            files.iter().map(|(p, c)| (p.clone(), c.clone())).collect();

        if let Some(in_flight_flag) = self.in_flight.get(rule_name) {
            in_flight_flag.store(true, Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(100));
        }

        if let Some(cached_issues) = self.cache.get(rule_name, &prompt_path, &files_owned) {
            (self.log_debug)(&format!("AI rule '{}' cache hit", rule_name));
            return Ok(self.validate_cached_issues(&cached_issues, files, workspace_root));
        }

        let prompt_content = std::fs::read_to_string(&prompt_path).map_err(AiError::IoError)?;

        let cancelled = Arc::new(AtomicBool::new(false));
        self.in_flight
            .insert(rule_name.to_string(), cancelled.clone());

        let result = self.call_ai_and_parse(
            rule_name,
            rule_config,
            &prompt_content,
            files,
            workspace_root,
            ai_config,
            changed_lines,
            &cancelled,
        );

        self.in_flight.remove(rule_name);

        if cancelled.load(Ordering::SeqCst) {
            (self.log_debug)(&format!("AI rule '{}' was cancelled", rule_name));
            return Ok(vec![]);
        }

        if let Ok(ref issues) = result {
            self.cache
                .insert(rule_name, &prompt_path, &files_owned, issues.clone());
        }

        result
    }

    #[allow(clippy::too_many_arguments)]
    fn call_ai_and_parse(
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
        let rule_prompt = prompt_content.replace(ai_placeholder_files(), &files_section);
        let options_section = if rule_config.options.is_null() {
            String::new()
        } else {
            format!(
                "## Additional Options\n\n```json\n{}\n```\n",
                serde_json::to_string_pretty(&rule_config.options).unwrap_or_default()
            )
        };
        let full_prompt = AI_RULE_WRAPPER
            .replace(ai_placeholder_content(), &rule_prompt)
            .replace(ai_placeholder_options(), &options_section);

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

        (self.log_debug)(&format!(
            "AI rule '{}': calling {} with {} files, mode={} (timeout: {}s)",
            rule_name,
            program,
            files.len(),
            mode_str,
            timeout_secs
        ));

        let response =
            self.spawn_ai_command(&program, &args, &full_prompt, timeout_ms, cancelled)?;

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

    fn spawn_ai_command(
        &self,
        program: &str,
        args: &[String],
        prompt: &str,
        timeout_ms: u64,
        cancelled: &Arc<AtomicBool>,
    ) -> Result<String, AiError> {
        (self.log_debug)(&format!(
            "Spawning: {} {:?} (cwd: {:?})",
            program, args, self.workspace_root
        ));

        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&self.workspace_root)
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    AiError::ProviderNotFound(program.to_string())
                } else {
                    AiError::IoError(e)
                }
            })?;

        let mut stdin = child.stdin.take().unwrap();
        let prompt_clone = prompt.to_string();
        let write_handle = std::thread::spawn(move || stdin.write_all(prompt_clone.as_bytes()));

        let timeout = if timeout_ms > 0 {
            Some(Duration::from_millis(timeout_ms))
        } else {
            None
        };
        let start = Instant::now();

        loop {
            if cancelled.load(Ordering::SeqCst) {
                let _ = child.kill();
                return Ok(String::new());
            }

            match child.try_wait() {
                Ok(Some(status)) => {
                    let _ = write_handle.join();

                    let mut stdout = Vec::new();
                    let mut stderr = Vec::new();

                    if let Some(mut stdout_handle) = child.stdout.take() {
                        let _ = stdout_handle.read_to_end(&mut stdout);
                    }
                    if let Some(mut stderr_handle) = child.stderr.take() {
                        let _ = stderr_handle.read_to_end(&mut stderr);
                    }

                    if !status.success() {
                        return Err(AiError::NonZeroExit {
                            code: status.code(),
                            stderr: String::from_utf8_lossy(&stderr).to_string(),
                        });
                    }

                    return Ok(String::from_utf8_lossy(&stdout).to_string());
                }
                Ok(None) => {
                    if let Some(t) = timeout {
                        if start.elapsed() > t {
                            let _ = child.kill();
                            return Err(AiError::Timeout(timeout_ms / 1000));
                        }
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => return Err(AiError::IoError(e)),
            }
        }
    }

    fn parse_response(
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

        let file_lines: HashMap<PathBuf, Vec<&str>> = files
            .iter()
            .map(|(path, content)| {
                let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                (relative.to_path_buf(), content.lines().collect())
            })
            .collect();

        Ok(ai_response
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
                        "AI returned unknown file: {} (not in input files)",
                        issue.file
                    ));
                    return None;
                }

                let line_text = file_lines
                    .get(&file_path)
                    .and_then(|lines| super::utils::extract_line_text(lines, issue.line));

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
            .collect())
    }

    fn validate_cached_issues(
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

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    pub fn flush_cache(&self) {
        self.cache.flush();
    }

    fn save_prompt_to_tmp(&self, rule_name: &str, prompt: &str) {
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

impl Default for AiExecutor {
    fn default() -> Self {
        Self::new(
            Path::new("."),
            None,
            None,
            Arc::new(AiCache::new()),
            None,
            None,
        )
    }
}
