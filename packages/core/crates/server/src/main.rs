use std::io::{self, BufRead, Write};

mod compression;
mod handlers;
mod lsp_server;
mod protocol;
mod state;

use compression::send_compressed_response;
use handlers::*;
use protocol::*;
use state::ServerState;

fn main() {
    core::init_logger("rust_server     ");

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--lsp" {
        core::log_info("Starting in LSP mode");
        if let Err(e) = lsp_server::run_lsp_server() {
            core::log_error(&format!("LSP server error: {}", e));
            std::process::exit(1);
        }
        return;
    }

    core::log_info("TScanner server started (JSON-RPC mode)");

    let mut state = ServerState::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                core::log_error(&format!("Failed to read line: {}", e));
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let request: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                core::log_error(&format!("Failed to parse request: {}", e));
                continue;
            }
        };

        let response = handle_request(request, &mut state);
        send_compressed_response(&mut stdout, response);

        process_file_events(&state, &mut stdout);
    }
}

fn handle_request(request: Request, state: &mut ServerState) -> Response {
    match request.method.as_str() {
        "scan" => match serde_json::from_value(request.params) {
            Ok(params) => handle_scan(request.id, params, state),
            Err(e) => Response {
                id: request.id,
                result: None,
                error: Some(format!("Invalid params: {}", e)),
            },
        },
        "watch" => match serde_json::from_value(request.params) {
            Ok(params) => handle_watch(request.id, params, state),
            Err(e) => Response {
                id: request.id,
                result: None,
                error: Some(format!("Invalid params: {}", e)),
            },
        },
        "scanFile" => match serde_json::from_value(request.params) {
            Ok(params) => handle_scan_file(request.id, params, state),
            Err(e) => Response {
                id: request.id,
                result: None,
                error: Some(format!("Invalid params: {}", e)),
            },
        },
        "getRulesMetadata" => handle_get_rules_metadata(request.id),
        "scanContent" => match serde_json::from_value(request.params) {
            Ok(params) => handle_scan_content(request.id, params, state),
            Err(e) => Response {
                id: request.id,
                result: None,
                error: Some(format!("Invalid params: {}", e)),
            },
        },
        "formatResults" => match serde_json::from_value(request.params) {
            Ok(params) => handle_format_results(request.id, params),
            Err(e) => Response {
                id: request.id,
                result: None,
                error: Some(format!("Invalid params: {}", e)),
            },
        },
        "clearCache" => handle_clear_cache(request.id, state),
        _ => Response {
            id: request.id,
            result: None,
            error: Some(format!("Unknown method: {}", request.method)),
        },
    }
}

fn process_file_events(state: &ServerState, stdout: &mut io::Stdout) {
    if let Some(watcher) = &state.watcher {
        while let Some(event) = watcher.try_recv() {
            core::log_debug(&format!("File event: {:?}", event));

            use core::FileEvent;
            match event {
                FileEvent::Modified(path) | FileEvent::Created(path) => {
                    if let Some(scanner) = &state.scanner {
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
                    state.cache.invalidate(&path);
                }
            }
        }
    }
}
