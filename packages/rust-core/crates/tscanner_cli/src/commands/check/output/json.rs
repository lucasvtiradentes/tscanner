use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::CliOutput;
use tscanner_diagnostics::ScanResult;

pub struct JsonRenderer;

impl OutputRenderer for JsonRenderer {
    fn render(&self, _ctx: &CheckContext, output: &CliOutput, _result: &ScanResult) {
        if let Some(json) = output.to_json() {
            println!("{}", json);
        }
    }
}
