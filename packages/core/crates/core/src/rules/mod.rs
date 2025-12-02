mod bug_prevention;
mod code_quality;
mod imports;
mod metadata;
mod style;
mod traits;
mod type_safety;
mod variables;

pub use metadata::{
    get_all_rule_metadata, get_allowed_options_for_rule, RuleCategory, RuleMetadata,
    RuleMetadataRegistration, RuleType,
};
pub use traits::{Rule, RuleRegistration};
