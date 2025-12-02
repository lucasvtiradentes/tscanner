use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};
use tscanner_config::{ScriptMode, ScriptRuleConfig};
use tscanner_diagnostics::{Issue, Severity};

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
    script_mtime: SystemTime,
}

#[derive(Debug)]
pub enum ScriptError {
    IoError(std::io::Error),
    Timeout(u64),
    NonZeroExit { code: Option<i32>, stderr: String },
    InvalidOutput(String),
    ScriptNotFound(PathBuf),
    RunnerNotFound(String),
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptError::IoError(e) => write!(f, "IO error: {}", e),
            ScriptError::Timeout(ms) => write!(f, "Script timed out after {}ms", ms),
            ScriptError::NonZeroExit { code, stderr } => {
                write!(f, "Script exited with code {:?}: {}", code, stderr)
            }
            ScriptError::InvalidOutput(msg) => write!(f, "Invalid script output: {}", msg),
            ScriptError::ScriptNotFound(path) => write!(f, "Script not found: {:?}", path),
            ScriptError::RunnerNotFound(runner) => write!(
                f,
                "Runner '{}' not found. Install it or specify a custom runner in the rule config.",
                runner
            ),
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
    scripts_dir: PathBuf,
    log_error: fn(&str),
    log_debug: fn(&str),
}

impl ScriptExecutor {
    pub fn new(workspace_root: &Path) -> Self {
        Self {
            cache: DashMap::new(),
            scripts_dir: workspace_root.join(".tscanner").join("scripts"),
            log_error: |_| {},
            log_debug: |_| {},
        }
    }

    pub fn with_logger(workspace_root: &Path, log_error: fn(&str), log_debug: fn(&str)) -> Self {
        Self {
            cache: DashMap::new(),
            scripts_dir: workspace_root.join(".tscanner").join("scripts"),
            log_error,
            log_debug,
        }
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
                            file: self.scripts_dir.join(&rule_config.script),
                            line: 0,
                            column: 0,
                            end_column: 0,
                            message: format!("Script error: {}", e),
                            severity: Severity::Error,
                            line_text: None,
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
        if !rule_config.base.enabled {
            return false;
        }

        let relative = path.strip_prefix(workspace_root).unwrap_or(path);
        let relative_str = relative.to_string_lossy();

        if !rule_config.base.include.is_empty() {
            let matches_include = rule_config
                .base
                .include
                .iter()
                .any(|pattern| glob_match::glob_match(pattern, &relative_str));
            if !matches_include {
                return false;
            }
        }

        if !rule_config.base.exclude.is_empty() {
            let matches_exclude = rule_config
                .base
                .exclude
                .iter()
                .any(|pattern| glob_match::glob_match(pattern, &relative_str));
            if matches_exclude {
                return false;
            }
        }

        true
    }

    fn execute_rule(
        &self,
        rule_name: &str,
        rule_config: &ScriptRuleConfig,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
    ) -> Result<Vec<Issue>, ScriptError> {
        let script_path = self.scripts_dir.join(&rule_config.script);

        if !script_path.exists() {
            return Err(ScriptError::ScriptNotFound(script_path));
        }

        let script_mtime = script_path
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let cache_key = self.compute_cache_key(rule_name, &script_path, files);

        if let Some(cached) = self.cache.get(&cache_key) {
            if cached.script_mtime == script_mtime {
                (self.log_debug)(&format!("Script rule '{}' cache hit", rule_name));
                return Ok(cached.issues.clone());
            }
        }

        let issues = match rule_config.mode {
            ScriptMode::Batch => {
                self.execute_batch(rule_name, rule_config, &script_path, files, workspace_root)?
            }
            ScriptMode::Single => self.execute_single_parallel(
                rule_name,
                rule_config,
                &script_path,
                files,
                workspace_root,
            )?,
        };

        self.cache.insert(
            cache_key,
            CachedResult {
                issues: issues.clone(),
                script_mtime,
            },
        );

        Ok(issues)
    }

    fn execute_batch(
        &self,
        rule_name: &str,
        rule_config: &ScriptRuleConfig,
        script_path: &Path,
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

        let input = ScriptInput {
            files: script_files,
            options: rule_config.options.clone(),
            workspace_root: workspace_root.to_string_lossy().to_string(),
        };

        let input_json = serde_json::to_vec(&input)
            .map_err(|e| ScriptError::InvalidOutput(format!("Failed to serialize input: {}", e)))?;

        let output = self.spawn_script(rule_config, script_path, &input_json, workspace_root)?;

        self.parse_output(rule_name, rule_config, &output, workspace_root)
    }

    fn execute_single_parallel(
        &self,
        rule_name: &str,
        rule_config: &ScriptRuleConfig,
        script_path: &Path,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
    ) -> Result<Vec<Issue>, ScriptError> {
        let results: Vec<Result<Vec<Issue>, ScriptError>> = files
            .par_iter()
            .map(|file| {
                self.execute_batch(
                    rule_name,
                    rule_config,
                    script_path,
                    &[*file],
                    workspace_root,
                )
            })
            .collect();

        let mut all_issues = Vec::new();
        for result in results {
            match result {
                Ok(issues) => all_issues.extend(issues),
                Err(e) => return Err(e),
            }
        }
        Ok(all_issues)
    }

    fn spawn_script(
        &self,
        rule_config: &ScriptRuleConfig,
        script_path: &Path,
        input: &[u8],
        cwd: &Path,
    ) -> Result<Vec<u8>, ScriptError> {
        let (program, args) = self.resolve_runner(rule_config, script_path)?;

        (self.log_debug)(&format!(
            "Running script: {} {:?} {:?}",
            program, args, script_path
        ));

        let mut child = Command::new(&program)
            .args(&args)
            .arg(script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(cwd)
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ScriptError::RunnerNotFound(program.clone())
                } else {
                    ScriptError::IoError(e)
                }
            })?;

        let mut stdin = child.stdin.take().unwrap();
        let input_clone = input.to_vec();
        let write_handle = std::thread::spawn(move || stdin.write_all(&input_clone));

        let timeout = Duration::from_millis(rule_config.timeout);
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

    fn resolve_runner(
        &self,
        rule_config: &ScriptRuleConfig,
        script_path: &Path,
    ) -> Result<(String, Vec<String>), ScriptError> {
        if let Some(runner) = &rule_config.runner {
            let parts: Vec<&str> = runner.split_whitespace().collect();
            if parts.is_empty() {
                return Err(ScriptError::RunnerNotFound(runner.clone()));
            }
            return Ok((
                parts[0].to_string(),
                parts[1..].iter().map(|s| s.to_string()).collect(),
            ));
        }

        let ext = script_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let (program, args): (&str, &[&str]) = match ext {
            "js" | "mjs" | "cjs" => ("node", &[]),
            "ts" | "mts" | "cts" => ("npx", &["tsx"]),
            "py" => ("python3", &[]),
            "sh" => ("bash", &[]),
            "rb" => ("ruby", &[]),
            "pl" => ("perl", &[]),
            "php" => ("php", &[]),
            "lua" => ("lua", &[]),
            _ => ("bash", &[]),
        };

        Ok((
            program.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
        ))
    }

    fn parse_output(
        &self,
        rule_name: &str,
        rule_config: &ScriptRuleConfig,
        output: &[u8],
        workspace_root: &Path,
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

        Ok(script_output
            .issues
            .into_iter()
            .map(|issue| {
                let file_path = workspace_root.join(&issue.file);
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
                    severity: rule_config.base.severity,
                    line_text: None,
                }
            })
            .collect())
    }

    fn compute_cache_key(
        &self,
        rule_name: &str,
        script_path: &Path,
        files: &[&(PathBuf, String)],
    ) -> u64 {
        let mut hasher = DefaultHasher::new();

        rule_name.hash(&mut hasher);
        script_path.hash(&mut hasher);

        if let Ok(script_content) = std::fs::read_to_string(script_path) {
            script_content.hash(&mut hasher);
        }

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
        Self::new(Path::new("."))
    }
}
