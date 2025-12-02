use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::Program;
use tscanner_diagnostics::Issue;

use crate::FileSource;

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

    fn is_regex_only(&self) -> bool {
        false
    }
}

pub struct RuleRegistration {
    pub name: &'static str,
    pub factory: fn(Option<&serde_json::Value>) -> Arc<dyn Rule>,
}

inventory::collect!(RuleRegistration);
