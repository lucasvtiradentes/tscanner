use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::FormattedOutput;
use tscanner_diagnostics::ScanResult;

pub struct JsonRenderer;

impl OutputRenderer for JsonRenderer {
    fn render(&self, _ctx: &CheckContext, output: &FormattedOutput, _result: &ScanResult) {
        if let Some(json) = output.to_json() {
            println!("{}", json);
        }
    }
}
