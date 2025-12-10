use crate::custom_requests::{FormatPrettyResult, FormatResultsParams, FormatSummary};
use lsp_server::{Connection, Message, Request, Response};
use std::collections::HashSet;
use tscanner_cli_output::{FormattedOutput, RulesBreakdown, SummaryStats};
use tscanner_types::Severity;

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_format_results(connection: &Connection, req: Request) -> Result<(), LspError> {
    let params: FormatResultsParams = serde_json::from_value(req.params)?;

    let unique_rules: HashSet<_> = params
        .results
        .files
        .iter()
        .flat_map(|f| &f.issues)
        .map(|i| &i.rule)
        .collect();
    let rule_count = unique_rules.len();

    let stats = SummaryStats::from_result(&params.results, rule_count, RulesBreakdown::default());

    let formatted = match params.group_mode.as_str() {
        "rule" => FormattedOutput::build_by_rule(&params.root, &params.results, &stats),
        _ => FormattedOutput::build_by_file(&params.root, &params.results, &stats),
    };

    let formatted_output = formatted.to_plain_text(true);

    let all_issues: Vec<_> = params
        .results
        .files
        .iter()
        .flat_map(|f| &f.issues)
        .collect();

    let error_count = all_issues
        .iter()
        .filter(|i| matches!(i.severity, Severity::Error))
        .count();
    let warning_count = all_issues
        .iter()
        .filter(|i| matches!(i.severity, Severity::Warning))
        .count();
    let info_count = all_issues
        .iter()
        .filter(|i| matches!(i.severity, Severity::Info))
        .count();
    let hint_count = all_issues
        .iter()
        .filter(|i| matches!(i.severity, Severity::Hint))
        .count();

    let file_count = params.results.files.len();

    let result = FormatPrettyResult {
        output: formatted_output,
        summary: FormatSummary {
            total_issues: params.results.total_issues,
            error_count,
            warning_count,
            info_count,
            hint_count,
            file_count,
            rule_count,
        },
    };

    let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
    connection.sender.send(Message::Response(response))?;

    Ok(())
}
