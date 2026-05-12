#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CliGroupBy {
    #[default]
    File,
    Rule,
}

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub group_by: CliGroupBy,
    pub show_settings: bool,
    pub show_summary: bool,
}

impl Default for CliOptions {
    fn default() -> Self {
        Self {
            group_by: CliGroupBy::File,
            show_settings: true,
            show_summary: true,
        }
    }
}
