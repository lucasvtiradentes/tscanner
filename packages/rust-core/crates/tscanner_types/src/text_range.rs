#[derive(Debug, Clone, PartialEq, Eq)]
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
