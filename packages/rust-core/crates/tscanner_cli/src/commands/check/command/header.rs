use std::path::Path;

use crate::shared::{print_section_header, render_header, ScanConfig, ScanMode};
use tscanner_cli::OutputFormat;
use tscanner_cli_output::GroupMode;
use tscanner_config::{AiExecutionMode, AiProvider};

pub(super) struct ScanHeaderParams<'a> {
    pub(super) root: &'a Path,
    pub(super) resolved_config_path: &'a str,
    pub(super) show_settings: bool,
    pub(super) scan_mode: ScanMode,
    pub(super) output_format: OutputFormat,
    pub(super) group_mode: GroupMode,
    pub(super) ai_mode: AiExecutionMode,
    pub(super) ai_provider: Option<AiProvider>,
    pub(super) cache_enabled: bool,
    pub(super) continue_on_error: bool,
    pub(super) glob_filter: Option<String>,
    pub(super) rule_filter: Option<String>,
    pub(super) severity_filter: Option<String>,
    pub(super) kind_filter: Option<String>,
}

pub(super) fn render_scan_header(params: ScanHeaderParams<'_>) {
    let relative_config_path = pathdiff::diff_paths(params.resolved_config_path, params.root)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| params.resolved_config_path.to_string());
    let scan_config = ScanConfig {
        show_settings: params.show_settings,
        mode: params.scan_mode,
        format: params.output_format,
        group_by: params.group_mode,
        ai_mode: params.ai_mode,
        ai_provider: params.ai_provider,
        cache_enabled: params.cache_enabled,
        continue_on_error: params.continue_on_error,
        config_path: relative_config_path,
        glob_filter: params.glob_filter,
        rule_filter: params.rule_filter,
        severity_filter: params.severity_filter,
        kind_filter: params.kind_filter,
    };
    render_header(&scan_config);
    print_section_header("Scanning...");
}
