use crate::config::TscannerConfig;
use crate::output::{FileResult, Issue};
use crate::scanning::{parse_file, RuleRegistry};
use crate::utils::{DisableDirectives, FileSource};
use std::path::Path;

const JS_TS_EXTENSIONS: &[&str] = &["ts", "tsx", "js", "jsx", "mjs", "cjs", "mts", "cts"];

pub fn is_js_ts_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| JS_TS_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

pub enum ExecuteResult {
    Skip,
    Disabled,
    ParseError,
    Ok(FileResult),
    Empty,
}

pub struct BuiltinExecutor<'a> {
    registry: &'a RuleRegistry,
    config: &'a TscannerConfig,
    root: &'a Path,
}

impl<'a> BuiltinExecutor<'a> {
    pub fn new(registry: &'a RuleRegistry, config: &'a TscannerConfig, root: &'a Path) -> Self {
        Self {
            registry,
            config,
            root,
        }
    }

    pub fn execute(&self, path: &Path, source: &str) -> ExecuteResult {
        if !is_js_ts_file(path) {
            return ExecuteResult::Skip;
        }

        let directives = DisableDirectives::from_source(source);

        if directives.file_disabled {
            return ExecuteResult::Disabled;
        }

        let file_source = FileSource::from_path(path);

        let program = match parse_file(path, source) {
            Ok(p) => p,
            Err(e) => {
                crate::utils::log_debug(&format!("Failed to parse {:?}: {}", path, e));
                return ExecuteResult::ParseError;
            }
        };

        let enabled_rules = self
            .registry
            .get_enabled_rules(path, self.root, self.config);
        let source_lines: Vec<&str> = source.lines().collect();

        let issues: Vec<Issue> = enabled_rules
            .iter()
            .filter(|(rule, _)| !(rule.is_typescript_only() && file_source.is_javascript()))
            .flat_map(|(rule, severity)| {
                let mut rule_issues = rule.check(&program, path, source, file_source);
                for issue in &mut rule_issues {
                    issue.severity = *severity;
                    if issue.line > 0 && issue.line <= source_lines.len() {
                        issue.line_text = Some(source_lines[issue.line - 1].to_string());
                    }
                }
                rule_issues
            })
            .filter(|issue| !directives.is_rule_disabled(issue.line, &issue.rule))
            .collect();

        if issues.is_empty() {
            ExecuteResult::Empty
        } else {
            ExecuteResult::Ok(FileResult {
                file: path.to_path_buf(),
                issues,
            })
        }
    }
}
