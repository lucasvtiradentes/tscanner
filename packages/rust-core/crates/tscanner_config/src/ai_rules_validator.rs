use crate::validation::ValidationResult;
use crate::AiRuleSourceConfig;
use std::path::Path;

pub fn validate_ai_rules(
    ai_rules: &[AiRuleSourceConfig],
    workspace: Option<&Path>,
    _config_dir_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::new();

    let Some(workspace) = workspace else {
        return result;
    };

    for (index, source) in ai_rules.iter().enumerate() {
        if source.path.trim().is_empty() {
            result.add_error(format!("AI rule source at index {} has empty path", index));
            continue;
        }

        let source_path = workspace.join(&source.path);
        if !source_path.exists() {
            result.add_warning(format!(
                "AI rule source '{}' was not found",
                source_path.display()
            ));
        }
    }

    result
}
