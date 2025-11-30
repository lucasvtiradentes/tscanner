use crate::output::Issue;
use crate::utils::FileSource;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::Program;

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        file_source: FileSource,
    ) -> Vec<Issue>;

    fn is_typescript_only(&self) -> bool {
        false
    }
}

pub struct RuleRegistration {
    pub name: &'static str,
    pub factory: fn() -> Arc<dyn Rule>,
}

inventory::collect!(RuleRegistration);

mod consistent_return;
mod max_function_length;
mod max_params;
mod metadata;
mod no_absolute_imports;
mod no_alias_imports;
mod no_any_type;
mod no_async_without_await;
mod no_console_log;
mod no_constant_condition;
mod no_default_export;
mod no_duplicate_imports;
mod no_dynamic_import;
mod no_else_return;
mod no_empty_class;
mod no_empty_function;
mod no_empty_interface;
mod no_floating_promises;
mod no_forwarded_exports;
mod no_implicit_any;
mod no_inferrable_types;
mod no_nested_require;
mod no_nested_ternary;
mod no_non_null_assertion;
mod no_relative_imports;
mod no_return_await;
mod no_shadow;
mod no_single_or_array_union;
mod no_todo_comments;
mod no_unnecessary_type_assertion;
mod no_unreachable_code;
mod no_unused_vars;
mod no_useless_catch;
mod no_var;
mod prefer_const;
mod prefer_interface_over_type;
mod prefer_nullish_coalescing;
mod prefer_optional_chain;
mod prefer_type_over_interface;
mod regex_rule;

pub use metadata::{
    get_all_rule_metadata, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType,
};
pub use regex_rule::RegexRule;
