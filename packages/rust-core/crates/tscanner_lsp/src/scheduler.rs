use lsp_server::{Message, Notification};
use lsp_types::{PublishDiagnosticsParams, Url};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::converters::issue_to_diagnostic;
use tscanner_service::{log_debug, ScanContentParams, Workspace, WorkspaceServer};

const DEFAULT_DEBOUNCE_MS: u64 = 150;

pub struct AnalysisRequest {
    pub uri: Url,
    pub path: PathBuf,
    pub content: String,
    pub version: i32,
}

struct PendingAnalysis {
    request: AnalysisRequest,
    scheduled_at: Instant,
}

pub struct AnalysisScheduler {
    sender: Sender<SchedulerCommand>,
    _worker: JoinHandle<()>,
}

enum SchedulerCommand {
    Schedule(AnalysisRequest),
    Cancel(Url),
    Shutdown,
}

impl AnalysisScheduler {
    pub fn new(
        connection_sender: crossbeam_channel::Sender<Message>,
        workspace: Arc<Mutex<WorkspaceServer>>,
    ) -> Self {
        Self::with_debounce(DEFAULT_DEBOUNCE_MS, connection_sender, workspace)
    }

    pub fn with_debounce(
        debounce_ms: u64,
        connection_sender: crossbeam_channel::Sender<Message>,
        workspace: Arc<Mutex<WorkspaceServer>>,
    ) -> Self {
        let pending: Arc<Mutex<HashMap<Url, PendingAnalysis>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_clone = pending.clone();

        let (tx, rx) = channel::<SchedulerCommand>();

        let worker = thread::spawn(move || {
            let debounce = Duration::from_millis(debounce_ms);

            loop {
                match rx.recv_timeout(Duration::from_millis(50)) {
                    Ok(SchedulerCommand::Schedule(request)) => {
                        let mut pending_guard = pending_clone.lock().unwrap();
                        pending_guard.insert(
                            request.uri.clone(),
                            PendingAnalysis {
                                request,
                                scheduled_at: Instant::now(),
                            },
                        );
                    }
                    Ok(SchedulerCommand::Cancel(uri)) => {
                        let mut pending_guard = pending_clone.lock().unwrap();
                        pending_guard.remove(&uri);
                    }
                    Ok(SchedulerCommand::Shutdown) => {
                        break;
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        break;
                    }
                }

                let ready_requests: Vec<AnalysisRequest> = {
                    let mut pending_guard = pending_clone.lock().unwrap();
                    let now = Instant::now();
                    let ready_uris: Vec<Url> = pending_guard
                        .iter()
                        .filter(|(_, analysis)| {
                            now.duration_since(analysis.scheduled_at) >= debounce
                        })
                        .map(|(uri, _)| uri.clone())
                        .collect();

                    ready_uris
                        .into_iter()
                        .filter_map(|uri| pending_guard.remove(&uri).map(|a| a.request))
                        .collect()
                };

                for request in ready_requests {
                    Self::run_analysis(&workspace, &connection_sender, request);
                }
            }
        });

        Self {
            sender: tx,
            _worker: worker,
        }
    }

    pub fn schedule(&self, request: AnalysisRequest) {
        let _ = self.sender.send(SchedulerCommand::Schedule(request));
    }

    pub fn cancel(&self, uri: &Url) {
        let _ = self.sender.send(SchedulerCommand::Cancel(uri.clone()));
    }

    fn run_analysis(
        workspace: &Arc<Mutex<WorkspaceServer>>,
        sender: &crossbeam_channel::Sender<Message>,
        request: AnalysisRequest,
    ) {
        let scan_result = {
            let ws = workspace.lock().unwrap();
            ws.scan_content(ScanContentParams {
                path: request.path.clone(),
                content: request.content.clone(),
            })
        };

        let issues = match &scan_result {
            Ok(result) => {
                log_debug(&format!(
                    "scheduler: {} issues for {}",
                    result.issues.len(),
                    request.path.display()
                ));
                result.issues.clone()
            }
            Err(e) => {
                log_debug(&format!("scheduler error: {:?}", e));
                Vec::new()
            }
        };

        let diagnostics = issues.iter().map(issue_to_diagnostic).collect();

        let params = PublishDiagnosticsParams {
            uri: request.uri,
            diagnostics,
            version: Some(request.version),
        };

        let notif = Notification {
            method: "textDocument/publishDiagnostics".to_string(),
            params: serde_json::to_value(params).unwrap(),
        };

        let _ = sender.send(Message::Notification(notif));
    }
}

impl Drop for AnalysisScheduler {
    fn drop(&mut self) {
        let _ = self.sender.send(SchedulerCommand::Shutdown);
    }
}
