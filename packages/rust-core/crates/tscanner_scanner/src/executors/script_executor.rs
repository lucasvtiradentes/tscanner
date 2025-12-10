use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tscanner_config::ScriptRuleConfig;
use tscanner_constants::config_dir_name;
use tscanner_types::{Issue, RuleSource, Severity};

#[derive(Debug, Clone, Serialize)]
pub struct ScriptFile {
    pub path: String,
    pub content: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptInput {
    pub files: Vec<ScriptFile>,
    pub options: Option<serde_json::Value>,
    pub workspace_root: String,
}

#[derive(Debug, Deserialize)]
pub struct ScriptIssue {
    pub file: String,
    pub line: usize,
    #[serde(default)]
    pub column: usize,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ScriptOutput {
    pub issues: Vec<ScriptIssue>,
}

#[derive(Debug, Clone)]
struct CachedResult {
    issues: Vec<Issue>,
}

#[derive(Debug)]
pub enum ScriptError {
    IoError(std::io::Error),
    Timeout(u64),
    NonZeroExit { code: Option<i32>, stderr: String },
    InvalidOutput(String),
    RunnerNotFound(String),
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptError::IoError(e) => write!(f, "IO error: {}", e),
            ScriptError::Timeout(ms) => write!(f, "Command timed out after {}ms", ms),
            ScriptError::NonZeroExit { code, stderr } => {
                write!(f, "Command exited with code {:?}: {}", code, stderr)
            }
            ScriptError::InvalidOutput(msg) => write!(f, "Invalid output: {}", msg),
            ScriptError::RunnerNotFound(cmd) => {
                write!(f, "Command '{}' not found", cmd)
            }
        }
    }
}

impl From<std::io::Error> for ScriptError {
    fn from(e: std::io::Error) -> Self {
        ScriptError::IoError(e)
    }
}

pub struct ScriptExecutor {
    cache: DashMap<u64, CachedResult>,
    config_dir: PathBuf,
    log_error: fn(&str),
    log_debug: fn(&str),
}

impl ScriptExecutor {
    pub fn new(
        workspace_root: &Path,
        config_dir: Option<PathBuf>,
        log_error: Option<fn(&str)>,
        log_debug: Option<fn(&str)>,
    ) -> Self {
        Self {
            cache: DashMap::new(),
            config_dir: config_dir.unwrap_or_else(|| workspace_root.join(config_dir_name())),
            log_error: log_error.unwrap_or(|_| {}),
            log_debug: log_debug.unwrap_or(|_| {}),
        }
    }

    pub fn with_config_dir(config_dir: PathBuf) -> Self {
        Self::new(Path::new("."), Some(config_dir), None, None)
    }

    pub fn with_logger(workspace_root: &Path, log_error: fn(&str), log_debug: fn(&str)) -> Self {
        Self::new(workspace_root, None, Some(log_error), Some(log_debug))
    }

    pub fn with_config_dir_and_logger(
        config_dir: PathBuf,
        log_error: fn(&str),
        log_debug: fn(&str),
    ) -> Self {
        Self::new(
            Path::new("."),
            Some(config_dir),
            Some(log_error),
            Some(log_debug),
        )
    }

    pub fn execute_rules(
        &self,
        rules: &[(String, ScriptRuleConfig)],
        all_files: &[(PathBuf, String)],
        workspace_root: &Path,
    ) -> Vec<Issue> {
        rules
            .par_iter()
            .flat_map(|(rule_name, rule_config)| {
                let matching_files: Vec<_> = all_files
                    .iter()
                    .filter(|(path, _)| self.file_matches_rule(path, workspace_root, rule_config))
                    .collect();

                if matching_files.is_empty() {
                    return vec![];
                }

                match self.execute_rule(rule_name, rule_config, &matching_files, workspace_root) {
                    Ok(issues) => issues,
                    Err(e) => {
                        (self.log_error)(&format!("Script rule '{}' failed: {}", rule_name, e));
                        vec![Issue {
                            rule: rule_name.clone(),
                            file: self.config_dir.clone(),
                            line: 0,
                            column: 0,
                            end_column: 0,
                            message: format!("Script error: {}", e),
                            severity: Severity::Error,
                            line_text: None,
                            category: None,
                            rule_type: RuleSource::CustomScript,
                        }]
                    }
                }
            })
            .collect()
    }

    fn file_matches_rule(
        &self,
        path: &Path,
        workspace_root: &Path,
        rule_config: &ScriptRuleConfig,
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
        rule_config: &ScriptRuleConfig,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
    ) -> Result<Vec<Issue>, ScriptError> {
        let cache_key = self.compute_cache_key(rule_name, &rule_config.command, files);

        if let Some(cached) = self.cache.get(&cache_key) {
            (self.log_debug)(&format!("Script rule '{}' cache hit", rule_name));
            return Ok(cached.issues.clone());
        }

        let issues = self.execute_batch(rule_name, rule_config, files, workspace_root)?;

        self.cache.insert(
            cache_key,
            CachedResult {
                issues: issues.clone(),
            },
        );

        Ok(issues)
    }

    fn execute_batch(
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

    fn spawn_command(
        &self,
        rule_config: &ScriptRuleConfig,
        input: &[u8],
    ) -> Result<Vec<u8>, ScriptError> {
        let parts: Vec<&str> = rule_config.command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ScriptError::RunnerNotFound(rule_config.command.clone()));
        }

        let program = parts[0];
        let args = &parts[1..];

        (self.log_debug)(&format!(
            "Running command: {} {:?} (cwd: {:?})",
            program, args, self.config_dir
        ));

        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&self.config_dir)
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ScriptError::RunnerNotFound(program.to_string())
                } else {
                    ScriptError::IoError(e)
                }
            })?;

        let mut stdin = child.stdin.take().unwrap();
        let input_clone = input.to_vec();
        let write_handle = std::thread::spawn(move || stdin.write_all(&input_clone));

        let timeout = Duration::from_secs(rule_config.timeout);
        let start = Instant::now();

        loop {
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
                        return Err(ScriptError::NonZeroExit {
                            code: status.code(),
                            stderr: String::from_utf8_lossy(&stderr).to_string(),
                        });
                    }

                    return Ok(stdout);
                }
                Ok(None) => {
                    if start.elapsed() > timeout {
                        let _ = child.kill();
                        return Err(ScriptError::Timeout(rule_config.timeout));
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) => return Err(ScriptError::IoError(e)),
            }
        }
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

        let json_start = output_str.find('{');
        let json_str = match json_start {
            Some(start) => &output_str[start..],
            None => {
                if output_str.trim().is_empty() {
                    return Ok(vec![]);
                }
                return Err(ScriptError::InvalidOutput(format!(
                    "No JSON found in output: {}",
                    output_str.chars().take(200).collect::<String>()
                )));
            }
        };

        let script_output: ScriptOutput = serde_json::from_str(json_str).map_err(|e| {
            ScriptError::InvalidOutput(format!(
                "Failed to parse JSON: {} - Output: {}",
                e,
                json_str.chars().take(500).collect::<String>()
            ))
        })?;

        use std::collections::HashMap;
        let file_lines: HashMap<PathBuf, Vec<&str>> = files
            .iter()
            .map(|(path, content)| {
                let relative = path.strip_prefix(workspace_root).unwrap_or(path);
                (relative.to_path_buf(), content.lines().collect())
            })
            .collect();

        Ok(script_output
            .issues
            .into_iter()
            .map(|issue| {
                let file_path = workspace_root.join(&issue.file);
                let relative_path = PathBuf::from(&issue.file);
                let line_text = file_lines
                    .get(&relative_path)
                    .and_then(|lines| super::utils::extract_line_text(lines, issue.line));
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
                    rule_type: RuleSource::CustomScript,
                }
            })
            .collect())
    }

    fn compute_cache_key(
        &self,
        rule_name: &str,
        command: &str,
        files: &[&(PathBuf, String)],
    ) -> u64 {
        let mut hasher = DefaultHasher::new();

        rule_name.hash(&mut hasher);
        command.hash(&mut hasher);

        let mut sorted: Vec<_> = files.iter().map(|(p, c)| (p, c)).collect();
        sorted.sort_by_key(|(p, _)| *p);

        for (path, content) in sorted {
            path.hash(&mut hasher);
            content.hash(&mut hasher);
        }

        hasher.finish()
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }
}

impl Default for ScriptExecutor {
    fn default() -> Self {
        Self::new(Path::new("."), None, None, None)
    }
}
