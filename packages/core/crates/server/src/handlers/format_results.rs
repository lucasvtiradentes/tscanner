use crate::protocol::{FormatResultsParams, Response};
use core::PrettyFormatter;

pub fn handle_format_results(id: u64, params: FormatResultsParams) -> Response {
    let formatted_output = match params.group_mode.as_str() {
        "rule" => PrettyFormatter::format_by_rule(&params.results, &params.root),
        _ => PrettyFormatter::format_by_file(&params.results, &params.root),
    };

    let error_count = params
        .results
        .files
        .iter()
        .flat_map(|f| &f.issues)
        .filter(|i| matches!(i.severity, core::Severity::Error))
        .count();

    let warning_count = params
        .results
        .files
        .iter()
        .flat_map(|f| &f.issues)
        .filter(|i| matches!(i.severity, core::Severity::Warning))
        .count();

    let file_count = params.results.files.len();

    let unique_rules: std::collections::HashSet<_> = params
        .results
        .files
        .iter()
        .flat_map(|f| &f.issues)
        .map(|i| &i.rule)
        .collect();
    let rule_count = unique_rules.len();

    Response {
        id,
        result: Some(serde_json::json!({
            "output": formatted_output,
            "summary": {
                "total_issues": params.results.total_issues,
                "error_count": error_count,
                "warning_count": warning_count,
                "file_count": file_count,
                "rule_count": rule_count
            }
        })),
        error: None,
    }
}
