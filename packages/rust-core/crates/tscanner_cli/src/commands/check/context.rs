use super::command::CliOptions;

#[derive(Clone)]
pub struct CheckContext {
    pub cli_options: CliOptions,
}

impl CheckContext {
    pub fn new(cli_options: CliOptions) -> Self {
        Self { cli_options }
    }
}
