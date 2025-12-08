use std::path::Path;
use tscanner_diagnostics::{Issue, IssueRuleType, Severity};

#[derive(Debug, Clone)]
pub struct TextRange {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl TextRange {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    pub fn single_line(line: usize, start_col: usize, end_col: usize) -> Self {
        Self {
            start_line: line,
            start_col,
            end_line: line,
            end_col,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuleDiagnostic {
    pub range: TextRange,
    pub message: String,
    pub severity: Severity,
    pub note: Option<String>,
}

impl RuleDiagnostic {
    pub fn new(range: TextRange, message: impl Into<String>) -> Self {
        Self {
            range,
            message: message.into(),
            severity: Severity::Warning,
            note: None,
        }
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: TextRange,
    pub new_text: String,
}

impl TextEdit {
    pub fn new(range: TextRange, new_text: impl Into<String>) -> Self {
        Self {
            range,
            new_text: new_text.into(),
        }
    }

    pub fn replace_line_segment(
        line: usize,
        start_col: usize,
        end_col: usize,
        new_text: impl Into<String>,
    ) -> Self {
        Self {
            range: TextRange::single_line(line, start_col, end_col),
            new_text: new_text.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionKind {
    QuickFix,
    Refactor,
    Source,
}

#[derive(Debug, Clone)]
pub struct RuleAction {
    pub title: String,
    pub kind: ActionKind,
    pub edits: Vec<TextEdit>,
}

impl RuleAction {
    pub fn quick_fix(title: impl Into<String>, edits: Vec<TextEdit>) -> Self {
        Self {
            title: title.into(),
            kind: ActionKind::QuickFix,
            edits,
        }
    }

    pub fn refactor(title: impl Into<String>, edits: Vec<TextEdit>) -> Self {
        Self {
            title: title.into(),
            kind: ActionKind::Refactor,
            edits,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuleSignal {
    pub diagnostic: RuleDiagnostic,
    pub action: Option<RuleAction>,
}

impl RuleSignal {
    pub fn new(diagnostic: RuleDiagnostic) -> Self {
        Self {
            diagnostic,
            action: None,
        }
    }

    pub fn with_action(mut self, action: RuleAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn to_issue(&self, rule_name: &str, path: &Path) -> Issue {
        Issue {
            rule: rule_name.to_string(),
            file: path.to_path_buf(),
            line: self.diagnostic.range.start_line,
            column: self.diagnostic.range.start_col,
            end_column: self.diagnostic.range.end_col,
            message: self.diagnostic.message.clone(),
            severity: self.diagnostic.severity,
            line_text: None,
            is_ai: false,
            category: None,
            rule_type: IssueRuleType::Builtin,
        }
    }
}
