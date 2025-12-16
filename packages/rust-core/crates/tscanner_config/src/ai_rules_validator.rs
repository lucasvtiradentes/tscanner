use std::path::Path;

use crate::validation::ValidationResult;
use crate::AiRuleConfig;
use tscanner_constants::{ai_placeholder_files, ai_placeholder_options, ai_rules_dir};

pub fn validate_ai_rules(
    ai_rules: &std::collections::HashMap<String, AiRuleConfig>,
    workspace: Option<&Path>,
    config_dir_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::new();

    let Some(workspace) = workspace else {
        return result;
    };

    let ai_rules_path = workspace.join(config_dir_name).join(ai_rules_dir());

    for (name, rule_config) in ai_rules {
        validate_single_rule(&mut result, name, rule_config, &ai_rules_path);
    }

    result
}

fn validate_single_rule(
    result: &mut ValidationResult,
    name: &str,
    rule_config: &AiRuleConfig,
    ai_rules_path: &Path,
) {
    let prompt_path = ai_rules_path.join(&rule_config.prompt);

    if !prompt_path.exists() {
        result.add_error(format!(
            "AI rule '{}': prompt file not found at '{}'",
            name,
            prompt_path.display()
        ));
        return;
    }

    let Ok(content) = std::fs::read_to_string(&prompt_path) else {
        result.add_error(format!(
            "AI rule '{}': failed to read prompt file '{}'",
            name,
            prompt_path.display()
        ));
        return;
    };

    if !content.contains(ai_placeholder_files()) {
        result.add_error(format!(
            "AI rule '{}': prompt file must contain '{}' placeholder",
            name,
            ai_placeholder_files()
        ));
    }

    let has_options_in_config = !rule_config.options.is_null();
    let has_options_placeholder = content.contains(ai_placeholder_options());

    if has_options_in_config && !has_options_placeholder {
        result.add_error(format!(
            "AI rule '{}': config has 'options' but prompt file is missing '{}' placeholder",
            name,
            ai_placeholder_options()
        ));
    }
}
