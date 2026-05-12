mod parser;
mod process;
mod prompt;
mod rule;
mod types;

use dashmap::DashMap;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tscanner_cache::AiCache;
use tscanner_config::{AiConfig, AiRuleConfig};
use tscanner_constants::{ai_rules_dir, config_dir_name};
use tscanner_types::Issue;

pub use types::{
    AiExecutionResult, AiProgressCallback, AiProgressEvent, AiRuleStatus, ChangedLinesMap,
    RegularRulesCompleteCallback,
};

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
    ) -> AiExecutionResult {
        self.execute_rules_with_progress(rules, files, workspace_root, changed_lines, None)
    }

    pub fn execute_rules_with_progress(
        &self,
        rules: &[(String, AiRuleConfig)],
        files: &[(PathBuf, String)],
        workspace_root: &Path,
        changed_lines: Option<&ChangedLinesMap>,
        progress_callback: Option<AiProgressCallback>,
    ) -> AiExecutionResult {
        if rules.is_empty() {
            return AiExecutionResult::default();
        }

        let ai_config = match &self.ai_config {
            Some(config) => config,
            None => {
                let error = format!(
                    "AI rules configured ({} rules) but 'ai' config section is missing. Add 'ai.provider' to your config.",
                    rules.len()
                );
                (self.log_warn)(&error);
                return AiExecutionResult {
                    errors: vec![error],
                    ..Default::default()
                };
            }
        };

        let total_rules = rules.len();
        let completed_count = Arc::new(AtomicUsize::new(0));
        let cache_hits = Arc::new(AtomicUsize::new(0));
        let errors: Arc<std::sync::Mutex<Vec<String>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));

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

        let cache_hits_ref = cache_hits.clone();
        let errors_ref = errors.clone();
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

                let (result, was_cache_hit) = self.execute_rule(
                    rule_name,
                    rule_config,
                    &matching_files,
                    workspace_root,
                    ai_config,
                    changed_lines,
                );

                if was_cache_hit {
                    cache_hits_ref.fetch_add(matching_files.len(), Ordering::SeqCst);
                }

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
                        let error_msg = format!("AI rule '{}' failed: {}", rule_name, e);
                        (self.log_warn)(&error_msg);
                        if let Ok(mut errs) = errors_ref.lock() {
                            errs.push(error_msg);
                        }
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

        let final_errors = errors.lock().map(|e| e.clone()).unwrap_or_default();
        AiExecutionResult {
            issues: all_issues,
            warnings: vec![],
            errors: final_errors,
            cache_hits: cache_hits.load(Ordering::SeqCst),
        }
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    pub fn flush_cache(&self) {
        self.cache.flush();
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
