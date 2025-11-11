use crate::types::{Issue, Severity};
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};
use std::path::Path;
use regex::Regex;

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue>;
}

pub struct NoAnyTypeRule;

impl Rule for NoAnyTypeRule {
    fn name(&self) -> &str {
        "no-any-type"
    }

    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut visitor = AnyTypeVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct AnyTypeVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for AnyTypeVisitor<'a> {
    fn visit_ts_keyword_type(&mut self, n: &TsKeywordType) {
        if matches!(n.kind, TsKeywordTypeKind::TsAnyKeyword) {
            let span = n.span();
            let (line, column) = self.get_line_col(span.lo.0 as usize);

            self.issues.push(Issue {
                rule: "no-any-type".to_string(),
                file: self.path.clone(),
                line,
                column,
                message: "Found `: any` type annotation".to_string(),
                severity: Severity::Error,
            });
        }
        n.visit_children_with(self);
    }

    fn visit_ts_as_expr(&mut self, n: &TsAsExpr) {
        if let TsType::TsKeywordType(ref kw) = &*n.type_ann {
            if matches!(kw.kind, TsKeywordTypeKind::TsAnyKeyword) {
                let span = kw.span();
                let (line, column) = self.get_line_col(span.lo.0 as usize);

                self.issues.push(Issue {
                    rule: "no-any-type".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    message: "Found `as any` type assertion".to_string(),
                    severity: Severity::Error,
                });
            }
        }
        n.visit_children_with(self);
    }
}

impl<'a> AnyTypeVisitor<'a> {
    fn get_line_col(&self, byte_pos: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;

        for (i, ch) in self.source.char_indices() {
            if i >= byte_pos {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        (line, col)
    }
}

pub struct NoConsoleLogRule;

impl Rule for NoConsoleLogRule {
    fn name(&self) -> &str {
        "no-console-log"
    }

    fn check(&self, _program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let regex = Regex::new(r"console\.log\(").unwrap();
        let mut issues = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            if let Some(mat) = regex.find(line) {
                issues.push(Issue {
                    rule: self.name().to_string(),
                    file: path.to_path_buf(),
                    line: line_num + 1,
                    column: mat.start() + 1,
                    message: "Avoid using console.log in production code".to_string(),
                    severity: Severity::Warning,
                });
            }
        }

        issues
    }
}

pub struct NoRelativeImportsRule;

impl Rule for NoRelativeImportsRule {
    fn name(&self) -> &str {
        "no-relative-imports"
    }

    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut visitor = RelativeImportVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct RelativeImportVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for RelativeImportVisitor<'a> {
    fn visit_import_decl(&mut self, n: &ImportDecl) {
        let span = n.src.span;
        let import_start = span.lo.0 as usize;
        let import_end = span.hi.0 as usize;

        if import_start < self.source.len() && import_end <= self.source.len() {
            let src_slice = &self.source[import_start..import_end];
            if src_slice.trim_matches('"').trim_matches('\'').starts_with('.') {
                let (line, column) = self.get_line_col(import_start);

                self.issues.push(Issue {
                    rule: "no-relative-imports".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    message: format!("Use absolute imports with @ prefix instead of relative imports"),
                    severity: Severity::Warning,
                });
            }
        }
        n.visit_children_with(self);
    }
}

impl<'a> RelativeImportVisitor<'a> {
    fn get_line_col(&self, byte_pos: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;

        for (i, ch) in self.source.char_indices() {
            if i >= byte_pos {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        (line, col)
    }
}

pub struct PreferTypeOverInterfaceRule;

impl Rule for PreferTypeOverInterfaceRule {
    fn name(&self) -> &str {
        "prefer-type-over-interface"
    }

    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut visitor = InterfaceVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct InterfaceVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for InterfaceVisitor<'a> {
    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl) {
        let span = n.span();
        let (line, column) = self.get_line_col(span.lo.0 as usize);

        self.issues.push(Issue {
            rule: "prefer-type-over-interface".to_string(),
            file: self.path.clone(),
            line,
            column,
            message: format!("Prefer 'type' over 'interface' for '{}'", n.id.sym),
            severity: Severity::Warning,
        });

        n.visit_children_with(self);
    }
}

impl<'a> InterfaceVisitor<'a> {
    fn get_line_col(&self, byte_pos: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;

        for (i, ch) in self.source.char_indices() {
            if i >= byte_pos {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        (line, col)
    }
}
