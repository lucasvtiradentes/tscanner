use serde::de::DeserializeOwned;
use std::io::{self, BufRead, Write};

mod compression;
mod handlers;
mod lsp;
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
        if let Err(e) = lsp::run_lsp_server() {
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

fn parse_params<T: DeserializeOwned>(id: u64, params: serde_json::Value) -> Result<T, Response> {
    serde_json::from_value(params).map_err(|e| Response {
        id,
        result: None,
        error: Some(format!("Invalid params: {}", e)),
    })
}

fn handle_request(request: Request, state: &mut ServerState) -> Response {
    let id = request.id;
    match request.method.as_str() {
        "scan" => {
            parse_params(id, request.params).map_or_else(|e| e, |p| handle_scan(id, p, state))
        }
        "watch" => {
            parse_params(id, request.params).map_or_else(|e| e, |p| handle_watch(id, p, state))
        }
        "scanFile" => {
            parse_params(id, request.params).map_or_else(|e| e, |p| handle_scan_file(id, p, state))
        }
        "scanContent" => parse_params(id, request.params)
            .map_or_else(|e| e, |p| handle_scan_content(id, p, state)),
        "formatResults" => {
            parse_params(id, request.params).map_or_else(|e| e, |p| handle_format_results(id, p))
        }
        "getRulesMetadata" => handle_get_rules_metadata(id),
        "clearCache" => handle_clear_cache(id, state),
        _ => Response {
            id,
            result: None,
            error: Some(format!("Unknown method: {}", request.method)),
        },
    }
}

fn process_file_events(state: &ServerState, stdout: &mut io::Stdout) {
    let Some(watcher) = &state.watcher else {
        return;
    };

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
