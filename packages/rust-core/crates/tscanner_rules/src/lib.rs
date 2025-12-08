pub mod builtin;
pub mod context;
pub mod executors;
mod file_source;
mod metadata;
mod registry;
pub mod signals;
mod traits;
pub mod utils;

pub use context::RuleContext;
pub use executors::{RegexExecutor, RegexRule};
pub use file_source::{FileSource, Language, LanguageVariant};
pub use metadata::{
    get_all_rule_metadata, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleOption,
    RuleOptionSchema, RuleType,
};
pub use registry::RuleRegistry;
pub use signals::{ActionKind, RuleAction, RuleDiagnostic, RuleSignal, TextEdit, TextRange};
pub use traits::{DynRule, Rule, RuleRegistration};
