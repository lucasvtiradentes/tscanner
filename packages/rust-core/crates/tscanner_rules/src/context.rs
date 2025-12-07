use crate::signals::TextRange;
use crate::FileSource;
use std::path::Path;
use swc_ecma_ast::Program;

pub struct RuleContext<'a> {
    program: &'a Program,
    path: &'a Path,
    source: &'a str,
    file_source: FileSource,
    lines: Vec<&'a str>,
}

impl<'a> RuleContext<'a> {
    pub fn new(
        program: &'a Program,
        path: &'a Path,
        source: &'a str,
        file_source: FileSource,
    ) -> Self {
        let lines = source.lines().collect();
        Self {
            program,
            path,
            source,
            file_source,
            lines,
        }
    }

    pub fn program(&self) -> &Program {
        self.program
    }

    pub fn path(&self) -> &Path {
        self.path
    }

    pub fn source(&self) -> &str {
        self.source
    }

    pub fn file_source(&self) -> &FileSource {
        &self.file_source
    }

    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.lines.get(line.saturating_sub(1)).copied()
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_text(&self, range: &TextRange) -> String {
        if range.start_line == range.end_line {
            self.lines
                .get(range.start_line.saturating_sub(1))
                .map(|l| {
                    let start = range.start_col.saturating_sub(1);
                    let end = range.end_col.saturating_sub(1).min(l.len());
                    if start < end && start < l.len() {
                        l[start..end].to_string()
                    } else {
                        String::new()
                    }
                })
                .unwrap_or_default()
        } else {
            let mut result = String::new();
            for i in range.start_line..=range.end_line {
                if let Some(line) = self.lines.get(i.saturating_sub(1)) {
                    if i == range.start_line {
                        let start = range.start_col.saturating_sub(1).min(line.len());
                        result.push_str(&line[start..]);
                    } else if i == range.end_line {
                        let end = range.end_col.saturating_sub(1).min(line.len());
                        result.push_str(&line[..end]);
                    } else {
                        result.push_str(line);
                    }
                    if i != range.end_line {
                        result.push('\n');
                    }
                }
            }
            result
        }
    }
}
