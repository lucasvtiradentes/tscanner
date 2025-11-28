use std::env;

pub const APP_NAME: &str = "tscanner";
pub const APP_DISPLAY_NAME: &str = "TScanner";
pub const APP_DESCRIPTION: &str = "Code quality scanner for the AI-generated code era";

pub const CONFIG_DIR_NAME: &str = ".tscanner";
pub const CONFIG_FILE_NAME: &str = "config.jsonc";

pub const LOG_BASENAME: &str = "tscannerlogs";

pub fn is_dev_mode() -> bool {
    env::var("CI").is_err() && env::var("GITHUB_ACTIONS").is_err()
}

pub fn get_log_filename() -> String {
    if is_dev_mode() {
        format!("{}-dev.txt", LOG_BASENAME)
    } else {
        format!("{}.txt", LOG_BASENAME)
    }
}
