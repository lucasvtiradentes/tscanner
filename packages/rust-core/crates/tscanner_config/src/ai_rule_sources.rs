use crate::validation::ValidationResult;
use crate::{
    AiMode, AiRuleClassification, AiRuleSourceConfig, AiRuleSourceType, AiRuleSummary,
    ResolvedAiRuleConfig, Severity, TscannerConfig,
};
use serde::Deserialize;
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Frontmatter {
    id: Option<String>,
    description: Option<String>,
    message: Option<String>,
    globs: Option<StringOrVec>,
    paths: Option<StringOrVec>,
    include: Option<StringOrVec>,
    exclude: Option<StringOrVec>,
    #[serde(default)]
    always_apply: Option<bool>,
    mode: Option<AiMode>,
    severity: Option<Severity>,
    timeout: Option<u64>,
    options: Option<Value>,
    classification: Option<AiRuleClassification>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

#[derive(Default)]
struct Resolver {
    resolved: Vec<ResolvedAiRuleConfig>,
    warnings: Vec<String>,
    seen_ids: HashSet<String>,
    summary: AiRuleSummary,
}

pub fn resolve_ai_rule_sources(
    config: &mut TscannerConfig,
    workspace: Option<&Path>,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    let Some(workspace) = workspace else {
        return result;
    };

    let mut resolver = Resolver {
        summary: AiRuleSummary {
            source_count: config.ai_rules.len(),
            ..Default::default()
        },
        ..Default::default()
    };

    for source in &config.ai_rules {
        resolver.resolve_source(config, source, workspace);
    }

    config.resolved_ai_rules = resolver.resolved;
    config.ai_rule_summary = resolver.summary;

    if config.ai_rule_summary.source_count > 0 {
        result.add_warning(format!(
            "AI rules: {} sources, {} markdown files, {} code-checkable, {} guidance-only, {} unsupported, {} skipped",
            config.ai_rule_summary.source_count,
            config.ai_rule_summary.markdown_count,
            config.ai_rule_summary.code_checkable_count,
            config.ai_rule_summary.guidance_only_count,
            config.ai_rule_summary.unsupported_count,
            config.ai_rule_summary.skipped_count,
        ));
    }

    for warning in resolver.warnings {
        result.add_warning(warning);
    }

    result
}

impl Resolver {
    fn resolve_source(
        &mut self,
        config: &TscannerConfig,
        source: &AiRuleSourceConfig,
        workspace: &Path,
    ) {
        if source.path.trim().is_empty() {
            self.skip("AI rule source has empty path".to_string());
            return;
        }

        let source_path = workspace.join(&source.path);
        if !source_path.exists() {
            self.skip(format!(
                "AI rule source '{}' was not found",
                source_path.display()
            ));
            return;
        }

        if source_path.is_file() {
            self.resolve_file(config, source, workspace, &source_path);
            return;
        }

        let mut files = Vec::new();
        if let Err(error) = Self::collect_markdown_files(&source_path, &mut files) {
            self.skip(format!(
                "AI rule source '{}' could not be read: {}",
                source_path.display(),
                error
            ));
            return;
        }
        files.sort();

        for file_path in files {
            self.resolve_file(config, source, workspace, &file_path);
        }
    }

    fn resolve_file(
        &mut self,
        config: &TscannerConfig,
        source: &AiRuleSourceConfig,
        workspace: &Path,
        file_path: &Path,
    ) {
        if should_skip_file(file_path) {
            return;
        }

        if !is_supported_markdown(file_path) {
            self.skip(format!(
                "AI rule source file '{}' is not a markdown rule",
                file_path.display()
            ));
            return;
        }

        let Ok(content) = fs::read_to_string(file_path) else {
            self.skip(format!(
                "AI rule source file '{}' could not be read",
                file_path.display()
            ));
            return;
        };

        let (frontmatter_raw, body) = strip_frontmatter(&content);
        let frontmatter = parse_frontmatter(frontmatter_raw);
        let source_type = infer_source_type(&source.path, file_path);
        let title = extract_title(&body);
        let id = source
            .id
            .clone()
            .or_else(|| frontmatter.id.clone())
            .unwrap_or_else(|| stable_id(workspace, file_path, title.as_deref()));

        if matches_ignore(&source.ignore, file_path, &id) {
            self.summary.skipped_count += 1;
            return;
        }

        if !self.seen_ids.insert(id.clone()) {
            self.summary.skipped_count += 1;
            self.warnings.push(format!(
                "AI rule '{}' from '{}' skipped because another imported rule already used that id",
                id,
                file_path.display()
            ));
            return;
        }

        self.summary.markdown_count += 1;

        let include = resolve_include(config, source, &frontmatter);
        let exclude = resolve_exclude(source, &frontmatter);
        let default_classification = if !include.is_empty() {
            AiRuleClassification::CodeCheckable
        } else {
            AiRuleClassification::GuidanceOnly
        };
        let classification = source
            .classification
            .or(frontmatter.classification)
            .unwrap_or(default_classification);

        match classification {
            AiRuleClassification::CodeCheckable => self.summary.code_checkable_count += 1,
            AiRuleClassification::GuidanceOnly => self.summary.guidance_only_count += 1,
            AiRuleClassification::Unsupported => self.summary.unsupported_count += 1,
        }

        let mut include = include;
        if classification == AiRuleClassification::CodeCheckable && include.is_empty() {
            include = config.files.include.clone();
        }

        self.resolved.push(ResolvedAiRuleConfig {
            id,
            prompt_path: file_path.to_path_buf(),
            prompt_hash: hash_string(&content),
            message: source
                .message
                .clone()
                .or(frontmatter.message)
                .or(frontmatter.description)
                .or(title)
                .unwrap_or_else(|| {
                    file_path
                        .file_stem()
                        .and_then(|name| name.to_str())
                        .unwrap_or("AI rule")
                        .replace('-', " ")
                }),
            mode: source.mode.or(frontmatter.mode).unwrap_or_default(),
            severity: source.severity.or(frontmatter.severity).unwrap_or_default(),
            include,
            exclude,
            timeout: source.timeout.or(frontmatter.timeout).unwrap_or_default(),
            options: if !source.options.is_null() {
                source.options.clone()
            } else {
                frontmatter.options.unwrap_or(Value::Null)
            },
            source_path: source.path.clone(),
            file_path: file_path
                .strip_prefix(workspace)
                .unwrap_or(file_path)
                .display()
                .to_string(),
            source_type,
            classification,
        });
    }

    fn skip(&mut self, warning: String) {
        self.summary.skipped_count += 1;
        self.warnings.push(warning);
    }

    fn collect_markdown_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
        let entries = fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                Self::collect_markdown_files(&path, files)?;
            } else if file_type.is_file() && is_supported_markdown(&path) {
                files.push(path);
            }
        }

        Ok(())
    }
}

pub fn strip_frontmatter(content: &str) -> (Option<&str>, String) {
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content);

    let Some(rest) = normalized
        .strip_prefix("---\n")
        .or_else(|| normalized.strip_prefix("---\r\n"))
    else {
        return (None, normalized.to_string());
    };

    let Some((end, body_start)) = find_frontmatter_end(rest) else {
        return (None, normalized.to_string());
    };

    let frontmatter = &rest[..end];
    (Some(frontmatter), rest[body_start..].to_string())
}

fn find_frontmatter_end(content: &str) -> Option<(usize, usize)> {
    [
        "---\r\n",
        "---\n",
        "---",
        "\r\n---\r\n",
        "\r\n---\n",
        "\n---\r\n",
        "\n---\n",
        "\r\n---",
        "\n---",
    ]
    .iter()
    .filter_map(|marker| content.find(marker).map(|end| (end, end + marker.len())))
    .min_by_key(|(end, _)| *end)
}

fn parse_frontmatter(frontmatter: Option<&str>) -> Frontmatter {
    let Some(frontmatter) = frontmatter else {
        return Frontmatter::default();
    };

    serde_yaml::from_str(frontmatter).unwrap_or_default()
}

fn resolve_include(
    config: &TscannerConfig,
    source: &AiRuleSourceConfig,
    frontmatter: &Frontmatter,
) -> Vec<String> {
    if !source.include.is_empty() {
        return source.include.clone();
    }

    if let Some(include) = frontmatter.include.as_ref() {
        return include_to_vec(include);
    }

    if let Some(globs) = frontmatter.globs.as_ref() {
        return include_to_vec(globs);
    }

    if let Some(paths) = frontmatter.paths.as_ref() {
        return include_to_vec(paths);
    }

    if frontmatter.always_apply == Some(true) {
        return config.files.include.clone();
    }

    Vec::new()
}

fn resolve_exclude(source: &AiRuleSourceConfig, frontmatter: &Frontmatter) -> Vec<String> {
    if !source.exclude.is_empty() {
        return source.exclude.clone();
    }

    frontmatter
        .exclude
        .as_ref()
        .map(include_to_vec)
        .unwrap_or_default()
}

fn include_to_vec(value: &StringOrVec) -> Vec<String> {
    match value {
        StringOrVec::String(value) => split_pattern_string(value),
        StringOrVec::Vec(values) => values.clone(),
    }
}

fn split_pattern_string(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| item.trim_matches('"').trim_matches('\'').to_string())
        .collect()
}

fn infer_source_type(source_path: &str, file_path: &Path) -> AiRuleSourceType {
    let source = source_path.replace('\\', "/");
    let file = file_path.to_string_lossy().replace('\\', "/");
    if source.contains(".cursor/rules")
        || file_path.extension().and_then(|ext| ext.to_str()) == Some("mdc")
    {
        AiRuleSourceType::Cursor
    } else if source.contains(".claude/rules") || file.contains(".claude/rules") {
        AiRuleSourceType::Claude
    } else {
        AiRuleSourceType::Generic
    }
}

fn is_supported_markdown(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("md") | Some("mdc")
    )
}

fn should_skip_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "AGENTS.md")
}

fn matches_ignore(ignore: &[String], file_path: &Path, id: &str) -> bool {
    if ignore.is_empty() {
        return false;
    }

    let file_name = file_path.file_name().and_then(|name| name.to_str());
    let stem = file_path.file_stem().and_then(|name| name.to_str());
    ignore.iter().any(|item| {
        item == id
            || Some(item.as_str()) == file_name
            || Some(item.as_str()) == stem
            || file_path
                .components()
                .any(|part| part.as_os_str().to_string_lossy() == item.as_str())
    })
}

fn extract_title(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        line.strip_prefix("# ")
            .map(str::trim)
            .filter(|title| !title.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn stable_id(workspace: &Path, file_path: &Path, title: Option<&str>) -> String {
    if let Some(title) = title {
        return slugify(title);
    }

    let relative = file_path.strip_prefix(workspace).unwrap_or(file_path);
    let stem = relative
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("ai-rule");
    slugify(stem)
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_matches('-').to_string()
}

fn hash_string(value: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_workspace(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("tscanner-ai-rule-test-{}-{}", name, nonce));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn base_config(ai_rules: Vec<AiRuleSourceConfig>) -> TscannerConfig {
        TscannerConfig {
            schema: None,
            rules: Default::default(),
            ai_rules,
            resolved_ai_rules: Vec::new(),
            ai_rule_summary: Default::default(),
            files: crate::FilesConfig {
                include: vec!["**/*.ts".to_string()],
                exclude: vec!["**/node_modules/**".to_string()],
            },
        }
    }

    fn source(path: &str) -> AiRuleSourceConfig {
        AiRuleSourceConfig {
            path: path.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn resolves_cursor_globs_as_code_checkable() {
        let workspace = temp_workspace("cursor-globs");
        let rules_dir = workspace.join(".cursor/rules");
        fs::create_dir_all(&rules_dir).unwrap();
        fs::write(
            rules_dir.join("api.mdc"),
            "---\ndescription: API rule\nglobs:\n  - \"apps/**/*.ts\"\n---\n# API Rule\nReport bad API code.",
        )
        .unwrap();

        let mut config = base_config(vec![source(".cursor/rules")]);
        let result = resolve_ai_rule_sources(&mut config, Some(&workspace));

        assert!(result.errors.is_empty());
        assert_eq!(config.resolved_ai_rules.len(), 1);
        let rule = &config.resolved_ai_rules[0];
        assert_eq!(rule.source_type, AiRuleSourceType::Cursor);
        assert_eq!(rule.classification, AiRuleClassification::CodeCheckable);
        assert_eq!(rule.include, vec!["apps/**/*.ts"]);
        assert_eq!(rule.message, "API rule");
    }

    #[test]
    fn cursor_always_apply_uses_global_include() {
        let workspace = temp_workspace("always-apply");
        let rules_dir = workspace.join(".cursor/rules");
        fs::create_dir_all(&rules_dir).unwrap();
        fs::write(
            rules_dir.join("global.mdc"),
            "---\nalwaysApply: true\n---\n# Global Rule\nReport bad global code.",
        )
        .unwrap();

        let mut config = base_config(vec![source(".cursor/rules")]);
        let result = resolve_ai_rule_sources(&mut config, Some(&workspace));

        assert!(result.errors.is_empty());
        assert_eq!(config.resolved_ai_rules[0].include, vec!["**/*.ts"]);
        assert_eq!(
            config.resolved_ai_rules[0].classification,
            AiRuleClassification::CodeCheckable
        );
    }

    #[test]
    fn resolves_nested_rule_directories() {
        let workspace = temp_workspace("nested-rules");
        let rules_dir = workspace.join(".cursor/rules/frontend");
        fs::create_dir_all(&rules_dir).unwrap();
        fs::write(
            rules_dir.join("component.mdc"),
            "---\nglobs: \"apps/web/**/*.tsx\"\n---\n# Component Rule\nReport bad UI code.",
        )
        .unwrap();

        let mut config = base_config(vec![source(".cursor/rules")]);
        let result = resolve_ai_rule_sources(&mut config, Some(&workspace));

        assert!(result.errors.is_empty());
        assert_eq!(config.resolved_ai_rules.len(), 1);
        assert_eq!(
            config.resolved_ai_rules[0].file_path,
            ".cursor/rules/frontend/component.mdc"
        );
    }

    #[test]
    fn parses_crlf_frontmatter() {
        let content = "---\r\npaths: \"**/*.ts\"\r\n---\r\nRule";
        let (frontmatter, body) = strip_frontmatter(content);

        assert_eq!(frontmatter, Some("paths: \"**/*.ts\""));
        assert_eq!(body, "Rule");
    }

    #[test]
    fn parses_empty_frontmatter() {
        let (frontmatter, body) = strip_frontmatter("---\n---\nRule");

        assert_eq!(frontmatter, Some(""));
        assert_eq!(body, "Rule");
    }

    #[test]
    fn unscoped_generic_markdown_is_guidance_only() {
        let workspace = temp_workspace("guidance-only");
        fs::write(workspace.join("rule.md"), "# Rule\nGeneral guidance.").unwrap();

        let mut config = base_config(vec![source("rule.md")]);
        let result = resolve_ai_rule_sources(&mut config, Some(&workspace));

        assert!(result.errors.is_empty());
        assert_eq!(
            config.resolved_ai_rules[0].classification,
            AiRuleClassification::GuidanceOnly
        );
        assert_eq!(config.ai_rule_summary.guidance_only_count, 1);
    }

    #[test]
    fn source_override_wins_over_frontmatter() {
        let workspace = temp_workspace("override");
        fs::write(
            workspace.join("rule.md"),
            "---\nmessage: Frontmatter\npaths:\n  - \"a/**/*.ts\"\n---\n# Rule\nReport.",
        )
        .unwrap();

        let mut entry = source("rule.md");
        entry.message = Some("Config".to_string());
        entry.include = vec!["b/**/*.ts".to_string()];
        let mut config = base_config(vec![entry]);
        let result = resolve_ai_rule_sources(&mut config, Some(&workspace));

        assert!(result.errors.is_empty());
        assert_eq!(config.resolved_ai_rules[0].message, "Config");
        assert_eq!(config.resolved_ai_rules[0].include, vec!["b/**/*.ts"]);
    }

    #[test]
    fn duplicate_id_warns_and_skips_second_rule() {
        let workspace = temp_workspace("duplicate");
        let rules_dir = workspace.join("rules");
        fs::create_dir_all(&rules_dir).unwrap();
        fs::write(
            rules_dir.join("a.md"),
            "---\nid: same\npaths: \"**/*.ts\"\n---\n# A",
        )
        .unwrap();
        fs::write(
            rules_dir.join("b.md"),
            "---\nid: same\npaths: \"**/*.ts\"\n---\n# B",
        )
        .unwrap();

        let mut config = base_config(vec![source("rules")]);
        let result = resolve_ai_rule_sources(&mut config, Some(&workspace));

        assert!(result.errors.is_empty());
        assert_eq!(config.resolved_ai_rules.len(), 1);
        assert_eq!(config.ai_rule_summary.skipped_count, 1);
        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.contains("skipped")));
    }

    #[test]
    fn ignore_matches_file_name() {
        let workspace = temp_workspace("ignore");
        let rules_dir = workspace.join("rules");
        fs::create_dir_all(&rules_dir).unwrap();
        fs::write(rules_dir.join("ignored.md"), "# Ignored").unwrap();

        let mut entry = source("rules");
        entry.ignore = vec!["ignored.md".to_string()];
        let mut config = base_config(vec![entry]);
        let result = resolve_ai_rule_sources(&mut config, Some(&workspace));

        assert!(result.errors.is_empty());
        assert!(config.resolved_ai_rules.is_empty());
        assert_eq!(config.ai_rule_summary.skipped_count, 1);
    }
}
