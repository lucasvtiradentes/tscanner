use lino_core::{NoAnyTypeRule, Scanner};
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber;

#[derive(Debug, Deserialize)]
struct Request {
    id: u64,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct Response {
    id: u64,
    result: Option<serde_json::Value>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ScanParams {
    root: PathBuf,
}

fn main() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Lino server started");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to read line: {}", e);
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let request: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to parse request: {}", e);
                continue;
            }
        };

        let response = handle_request(request);

        if let Ok(json) = serde_json::to_string(&response) {
            if let Err(e) = writeln!(stdout, "{}", json) {
                error!("Failed to write response: {}", e);
            }
            if let Err(e) = stdout.flush() {
                error!("Failed to flush stdout: {}", e);
            }
        }
    }
}

fn handle_request(request: Request) -> Response {
    match request.method.as_str() {
        "scan" => {
            let params: ScanParams = match serde_json::from_value(request.params) {
                Ok(p) => p,
                Err(e) => {
                    return Response {
                        id: request.id,
                        result: None,
                        error: Some(format!("Invalid params: {}", e)),
                    }
                }
            };

            info!("Scanning workspace: {:?}", params.root);

            let scanner = Scanner::new(vec![Box::new(NoAnyTypeRule)]);
            let result = scanner.scan(&params.root);

            Response {
                id: request.id,
                result: Some(serde_json::to_value(&result).unwrap()),
                error: None,
            }
        }
        _ => Response {
            id: request.id,
            result: None,
            error: Some(format!("Unknown method: {}", request.method)),
        },
    }
}
