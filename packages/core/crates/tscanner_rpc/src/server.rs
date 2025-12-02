use crate::compression::send_compressed_response;
use crate::handlers::*;
use crate::protocol::{
    FormatResultsParams, Notification, Request, Response, ScanContentParams, ScanFileParams,
    ScanParams, WatchParams,
};
use crate::state::ServerState;
use std::io::{self, BufRead, Write};
use tscanner_fs::FileEvent;

pub struct RpcServer {
    state: ServerState,
}

impl RpcServer {
    pub fn new() -> Self {
        Self {
            state: ServerState::new(),
        }
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };

            if line.trim().is_empty() {
                continue;
            }

            let request: Request = match serde_json::from_str(&line) {
                Ok(r) => r,
                Err(_) => continue,
            };

            let response = self.handle_request(request);
            send_compressed_response(&mut stdout, response);

            self.process_file_events(&mut stdout);
        }
    }

    fn handle_request(&mut self, request: Request) -> Response {
        let id = request.id;
        match request.method.as_str() {
            "scan" => match serde_json::from_value::<ScanParams>(request.params) {
                Ok(p) => handle_scan(id, p, &mut self.state),
                Err(e) => Response::error(id, format!("Invalid params: {}", e)),
            },
            "scanFile" => match serde_json::from_value::<ScanFileParams>(request.params) {
                Ok(p) => handle_scan_file(id, p, &mut self.state),
                Err(e) => Response::error(id, format!("Invalid params: {}", e)),
            },
            "scanContent" => match serde_json::from_value::<ScanContentParams>(request.params) {
                Ok(p) => handle_scan_content(id, p, &mut self.state),
                Err(e) => Response::error(id, format!("Invalid params: {}", e)),
            },
            "formatResults" => {
                match serde_json::from_value::<FormatResultsParams>(request.params) {
                    Ok(p) => handle_format_results(id, p),
                    Err(e) => Response::error(id, format!("Invalid params: {}", e)),
                }
            }
            "getRulesMetadata" => handle_get_rules_metadata(id),
            "clearCache" => handle_clear_cache(id, &mut self.state),
            "watch" => match serde_json::from_value::<WatchParams>(request.params) {
                Ok(p) => handle_watch(id, p, &mut self.state),
                Err(e) => Response::error(id, format!("Invalid params: {}", e)),
            },
            _ => Response::error(id, format!("Unknown method: {}", request.method)),
        }
    }

    fn process_file_events(&self, stdout: &mut io::Stdout) {
        let Some(watcher) = &self.state.watcher else {
            return;
        };

        while let Some(event) = watcher.try_recv() {
            match event {
                FileEvent::Modified(path) | FileEvent::Created(path) => {
                    if let Some(scanner) = &self.state.scanner {
                        if let Some(result) = scanner.scan_single(&path) {
                            let notification = Notification {
                                method: "file_updated".to_string(),
                                params: serde_json::to_value(&result).unwrap(),
                            };
                            if let Ok(json) = serde_json::to_string(&notification) {
                                let _ = writeln!(stdout, "{}", json);
                                let _ = stdout.flush();
                            }
                        }
                    }
                }
                FileEvent::Removed(path) => {
                    self.state.cache.invalidate(&path);
                }
            }
        }
    }
}

impl Default for RpcServer {
    fn default() -> Self {
        Self::new()
    }
}
