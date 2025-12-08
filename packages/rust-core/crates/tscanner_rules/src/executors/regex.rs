use regex::Regex;
use tscanner_diagnostics::Severity;

use crate::context::RuleContext;
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::Rule;

pub struct RegexMatch {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct RegexExecutor {
    name: String,
    pattern: Regex,
    message: String,
    severity: Severity,
}

impl RegexExecutor {
    pub fn new(
        name: String,
        pattern: String,
        message: String,
        severity: Severity,
    ) -> Result<Self, regex::Error> {
        Ok(Self {
            name,
            pattern: Regex::new(&pattern)?,
            message,
            severity,
        })
    }

    fn static_name(&self) -> &'static str {
        Box::leak(self.name.clone().into_boxed_str())
    }
}

impl Rule for RegexExecutor {
    type State = RegexMatch;

    fn name(&self) -> &'static str {
        self.static_name()
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut matches = Vec::new();

        for (line_num, line) in ctx.source().lines().enumerate() {
            if let Some(mat) = self.pattern.find(line) {
                matches.push(RegexMatch {
                    line: line_num + 1,
                    start_col: mat.start() + 1,
                    end_col: mat.end() + 1,
                });
            }
        }

        matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            self.message.clone(),
        )
        .with_severity(self.severity)
    }

    fn is_regex_only(&self) -> bool {
        true
    }
}

pub type RegexRule = RegexExecutor;
