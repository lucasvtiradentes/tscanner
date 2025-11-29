use serde::Serialize;

#[derive(Serialize)]
pub struct JsonIssue {
    pub rule: String,
    pub severity: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
}

#[derive(Serialize)]
pub struct JsonFileGroup {
    pub file: String,
    pub issues: Vec<JsonIssue>,
}

#[derive(Serialize)]
pub struct JsonRuleGroup {
    pub rule: String,
    pub count: usize,
    pub issues: Vec<JsonRuleIssue>,
}

#[derive(Serialize)]
pub struct JsonRuleIssue {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
}
