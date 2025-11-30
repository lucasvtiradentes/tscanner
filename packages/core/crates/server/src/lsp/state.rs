use core::{FileCache, Scanner, TscannerConfig};
use lsp_types::{Diagnostic, Url};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub struct LspState {
    pub workspace_root: PathBuf,
    pub config: Option<TscannerConfig>,
    pub scanner: Option<Scanner>,
    pub open_files: HashMap<Url, String>,
    pub diagnostics: HashMap<Url, Vec<(Diagnostic, String)>>,
}

impl LspState {
    pub fn new(
        workspace_root: PathBuf,
        config: Option<TscannerConfig>,
        scanner: Option<Scanner>,
    ) -> Self {
        Self {
            workspace_root,
            config,
            scanner,
            open_files: HashMap::new(),
            diagnostics: HashMap::new(),
        }
    }

    pub fn reload_config(&mut self) -> Result<(), String> {
        match TscannerConfig::load_from_workspace(&self.workspace_root) {
            Ok(new_config) => {
                let config_hash = new_config.compute_hash();
                let cache = Arc::new(FileCache::with_config_hash(config_hash));

                match Scanner::with_cache(new_config.clone(), cache, self.workspace_root.clone()) {
                    Ok(new_scanner) => {
                        self.config = Some(new_config);
                        self.scanner = Some(new_scanner);
                        core::log_info("Config reloaded successfully");
                        Ok(())
                    }
                    Err(e) => {
                        self.config = None;
                        self.scanner = None;
                        Err(format!("Failed to create scanner: {}", e))
                    }
                }
            }
            Err(e) => {
                self.config = None;
                self.scanner = None;
                Err(format!("Config error: {}", e))
            }
        }
    }
}
