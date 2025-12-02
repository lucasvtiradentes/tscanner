use std::sync::Arc;
use tscanner_cache::FileCache;
use tscanner_fs::FileWatcher;
use tscanner_scanner::Scanner;

pub struct ServerState {
    pub scanner: Option<Scanner>,
    pub watcher: Option<FileWatcher>,
    pub cache: Arc<FileCache>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            scanner: None,
            watcher: None,
            cache: Arc::new(FileCache::new()),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}
