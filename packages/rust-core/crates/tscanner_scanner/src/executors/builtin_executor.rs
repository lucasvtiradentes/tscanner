use crate::disable_comments::DisableDirectives;
use crate::parser::parse_file;
use std::path::Path;
use tscanner_config::{CompiledRuleConfig, TscannerConfig};
use tscanner_diagnostics::{FileResult, Issue, IssueRuleType};
use tscanner_rules::{FileSource, RuleContext, RuleRegistry};

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
    log_debug_fn: fn(&str),
}

impl<'a> BuiltinExecutor<'a> {
    pub fn new(registry: &'a RuleRegistry, config: &'a TscannerConfig, root: &'a Path) -> Self {
        Self {
            registry,
            config,
            root,
            log_debug_fn: |_| {},
        }
    }

    pub fn with_logger(
        registry: &'a RuleRegistry,
        config: &'a TscannerConfig,
        root: &'a Path,
        log_debug: fn(&str),
    ) -> Self {
        Self {
            registry,
            config,
            root,
            log_debug_fn: log_debug,
        }
    }

    pub fn execute(&self, path: &Path, source: &str) -> ExecuteResult {
        if !is_js_ts_file(path) {
            return self.execute_regex_only(path, source);
        }

        let directives = DisableDirectives::from_source(source);

        if directives.is_file_fully_disabled() {
            return ExecuteResult::Disabled;
        }

        let file_source = FileSource::from_path(path);

        let program = match parse_file(path, source) {
            Ok(p) => p,
            Err(e) => {
                (self.log_debug_fn)(&format!("Failed to parse {:?}: {}", path, e));
                return ExecuteResult::ParseError;
            }
        };

        let enabled_rules = self.registry.get_enabled_rules(
            path,
            self.root,
            |file_path: &Path, root: &Path, compiled: &CompiledRuleConfig| {
                self.config
                    .matches_file_with_root(file_path, root, compiled)
            },
        );
        let source_lines: Vec<&str> = source.lines().collect();
        let ctx = RuleContext::new(&program, path, source, file_source);

        let issues: Vec<Issue> = enabled_rules
            .iter()
            .filter(|(rule, _)| !(rule.is_typescript_only() && file_source.is_javascript()))
            .flat_map(|(rule, severity)| {
                let signals = rule.signals(&ctx);
                let category = self.registry.get_rule_category(rule.name());
                signals
                    .into_iter()
                    .map(|signal| {
                        let mut issue = signal.to_issue(rule.name(), path);
                        issue.severity = *severity;
                        issue.category = category.map(|s| s.to_string());
                        issue.rule_type = if self.registry.is_custom_regex_rule(rule.name()) {
                            IssueRuleType::CustomRegex
                        } else {
                            IssueRuleType::Builtin
                        };
                        if issue.line > 0 && issue.line <= source_lines.len() {
                            issue.line_text = Some(source_lines[issue.line - 1].to_string());
                        }
                        issue
                    })
                    .collect::<Vec<_>>()
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

    fn execute_regex_only(&self, path: &Path, source: &str) -> ExecuteResult {
        let enabled_rules = self.registry.get_enabled_regex_rules(
            path,
            self.root,
            |file_path: &Path, root: &Path, compiled: &CompiledRuleConfig| {
                self.config
                    .matches_file_with_root(file_path, root, compiled)
            },
        );

        if enabled_rules.is_empty() {
            return ExecuteResult::Skip;
        }

        let source_lines: Vec<&str> = source.lines().collect();
        let file_source = FileSource::from_path(path);
        let empty_program = swc_ecma_ast::Program::Script(swc_ecma_ast::Script {
            span: swc_common::DUMMY_SP,
            body: vec![],
            shebang: None,
        });
        let ctx = RuleContext::new(&empty_program, path, source, file_source);

        let issues: Vec<Issue> = enabled_rules
            .iter()
            .flat_map(|(rule, severity)| {
                let signals = rule.signals(&ctx);
                let category = self.registry.get_rule_category(rule.name());
                signals
                    .into_iter()
                    .map(|signal| {
                        let mut issue = signal.to_issue(rule.name(), path);
                        issue.severity = *severity;
                        issue.category = category.map(|s| s.to_string());
                        issue.rule_type = if self.registry.is_custom_regex_rule(rule.name()) {
                            IssueRuleType::CustomRegex
                        } else {
                            IssueRuleType::Builtin
                        };
                        if issue.line > 0 && issue.line <= source_lines.len() {
                            issue.line_text = Some(source_lines[issue.line - 1].to_string());
                        }
                        issue
                    })
                    .collect::<Vec<_>>()
            })
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
