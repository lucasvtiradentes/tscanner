mod formatted;
mod plain_text;
mod types;

pub use formatted::{FormattedOutput, SummaryStats};
pub use types::{
    GroupMode, OutputFileGroup, OutputIssue, OutputRuleGroup, OutputRuleIssue, OutputSummary,
    RulesBreakdown,
};
