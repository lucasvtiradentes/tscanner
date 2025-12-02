use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Constants {
    package_name: String,
    package_display_name: String,
    package_description: String,
    config_dir_name: String,
    config_file_name: String,
    default_target_branch: String,
    log_basename: String,
    disable_file_comment: String,
    disable_next_line_comment: String,
}

const CONSTANTS_JSON: &str = include_str!("../../../../../assets/constants.json");

lazy_static::lazy_static! {
    static ref CONSTANTS: Constants = serde_json::from_str(CONSTANTS_JSON)
        .expect("Failed to parse constants.json");
}

pub fn app_name() -> &'static str {
    &CONSTANTS.package_name
}

pub fn app_display_name() -> &'static str {
    &CONSTANTS.package_display_name
}

pub fn app_description() -> &'static str {
    &CONSTANTS.package_description
}

pub fn config_dir_name() -> &'static str {
    &CONSTANTS.config_dir_name
}

pub fn config_file_name() -> &'static str {
    &CONSTANTS.config_file_name
}

pub fn default_target_branch() -> &'static str {
    &CONSTANTS.default_target_branch
}

pub fn log_basename() -> &'static str {
    &CONSTANTS.log_basename
}

pub fn is_dev_mode() -> bool {
    env::var("CI").is_err() && env::var("GITHUB_ACTIONS").is_err()
}

pub fn get_log_filename() -> String {
    if is_dev_mode() {
        format!("{}-dev.txt", log_basename())
    } else {
        format!("{}.txt", log_basename())
    }
}

pub fn disable_file_comment() -> &'static str {
    &CONSTANTS.disable_file_comment
}

pub fn disable_next_line_comment() -> &'static str {
    &CONSTANTS.disable_next_line_comment
}
