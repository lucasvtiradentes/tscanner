pub use tscanner_types::{RuleCategory, RuleMetadata, RuleOption, RuleOptionSchema, RuleType};

pub struct RuleMetadataRegistration {
    pub metadata: RuleMetadata,
}

inventory::collect!(RuleMetadataRegistration);

pub fn get_all_rule_metadata() -> Vec<RuleMetadata> {
    inventory::iter::<RuleMetadataRegistration>()
        .map(|reg| reg.metadata.clone())
        .collect()
}
