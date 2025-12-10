use tscanner_constants::{
    icon_ai, icon_builtin, icon_error, icon_hint, icon_info, icon_regex, icon_script, icon_warning,
};
use tscanner_types::IssueRuleType;

pub fn rule_type_icon(rule_type: IssueRuleType) -> &'static str {
    match rule_type {
        IssueRuleType::Builtin => icon_builtin(),
        IssueRuleType::CustomRegex => icon_regex(),
        IssueRuleType::CustomScript => icon_script(),
        IssueRuleType::Ai => icon_ai(),
    }
}

pub fn severity_icon(severity: &str) -> &'static str {
    match severity {
        "error" => icon_error(),
        "warning" => icon_warning(),
        "info" => icon_info(),
        "hint" => icon_hint(),
        _ => icon_warning(),
    }
}

pub fn format_duration(ms: u128) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        let total_seconds = ms / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    }
}
