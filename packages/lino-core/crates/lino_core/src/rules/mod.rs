use crate::types::Issue;
use swc_ecma_ast::Program;
use std::path::Path;
use std::sync::Arc;

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue>;
}

pub struct RuleRegistration {
    pub name: &'static str,
    pub factory: fn() -> Arc<dyn Rule>,
}

inventory::collect!(RuleRegistration);

mod metadata;
mod regex_rule;
mod no_any_type;
mod no_console_log;
mod no_relative_imports;
mod prefer_type_over_interface;
mod no_magic_numbers;

pub use metadata::{RuleMetadata, RuleMetadataRegistration, RuleCategory, get_all_rule_metadata};
pub use regex_rule::RegexRule;
