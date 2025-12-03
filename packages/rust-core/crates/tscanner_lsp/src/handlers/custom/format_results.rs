use crate::custom_requests::{FormatPrettyResult, FormatResultsParams, FormatSummary};
use lsp_server::{Connection, Message, Request, Response};
use std::collections::HashSet;
use tscanner_diagnostics::{PrettyFormatter, Severity};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_format_results(connection: &Connection, req: Request) -> Result<(), LspError> {
    let params: FormatResultsParams = serde_json::from_value(req.params)?;

    let formatted_output = match params.group_mode.as_str() {
        "rule" => PrettyFormatter::format_by_rule(&params.results, &params.root),
        _ => PrettyFormatter::format_by_file(&params.results, &params.root),
    };

    let error_count = params
        .results
        .files
        .iter()
        .flat_map(|f| &f.issues)
        .filter(|i| matches!(i.severity, Severity::Error))
        .count();

    let warning_count = params
        .results
        .files
        .iter()
        .flat_map(|f| &f.issues)
        .filter(|i| matches!(i.severity, Severity::Warning))
        .count();

    let file_count = params.results.files.len();

    let unique_rules: HashSet<_> = params
        .results
        .files
        .iter()
        .flat_map(|f| &f.issues)
        .map(|i| &i.rule)
        .collect();
    let rule_count = unique_rules.len();

    let result = FormatPrettyResult {
        output: formatted_output,
        summary: FormatSummary {
            total_issues: params.results.total_issues,
            error_count,
            warning_count,
            file_count,
            rule_count,
        },
    };

    let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
    connection.sender.send(Message::Response(response))?;

    Ok(())
}
