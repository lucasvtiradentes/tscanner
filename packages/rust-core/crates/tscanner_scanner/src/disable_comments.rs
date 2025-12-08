use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use tscanner_config::{ignore_comment, ignore_next_line_comment};

static IGNORE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"//\s*{}(?:\s+(.*))?$", ignore_comment())).unwrap());
static IGNORE_NEXT_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"//\s*{}\s+(.+)", ignore_next_line_comment())).unwrap());

#[derive(Debug, Clone)]
pub struct DisableDirectives {
    pub file_disabled_rules: HashSet<String>,
    pub line_disabled_rules: HashMap<usize, HashSet<String>>,
}

impl DisableDirectives {
    pub fn from_source(source: &str) -> Self {
        let mut file_disabled_rules: HashSet<String> = HashSet::new();
        let mut line_disabled_rules: HashMap<usize, HashSet<String>> = HashMap::new();

        for (line_num, line) in source.lines().enumerate() {
            let line_idx = line_num + 1;

            if let Some(caps) = IGNORE_NEXT_LINE_RE.captures(line) {
                if let Some(rules_str) = caps.get(1) {
                    let rules: HashSet<String> = rules_str
                        .as_str()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    line_disabled_rules.insert(line_idx + 1, rules);
                }
                continue;
            }

            if let Some(caps) = IGNORE_RE.captures(line) {
                if let Some(rules_str) = caps.get(1) {
                    let rules_text = rules_str.as_str().trim();
                    if rules_text.is_empty() {
                        file_disabled_rules.insert("*".to_string());
                    } else {
                        for rule in rules_text
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                        {
                            file_disabled_rules.insert(rule.to_string());
                        }
                    }
                } else {
                    file_disabled_rules.insert("*".to_string());
                }
            }
        }

        Self {
            file_disabled_rules,
            line_disabled_rules,
        }
    }

    pub fn is_rule_disabled(&self, line: usize, rule_name: &str) -> bool {
        if self.file_disabled_rules.contains("*") || self.file_disabled_rules.contains(rule_name) {
            return true;
        }

        if let Some(disabled_rules) = self.line_disabled_rules.get(&line) {
            return disabled_rules.contains(rule_name);
        }

        false
    }

    pub fn is_file_fully_disabled(&self) -> bool {
        self.file_disabled_rules.contains("*")
    }
}
