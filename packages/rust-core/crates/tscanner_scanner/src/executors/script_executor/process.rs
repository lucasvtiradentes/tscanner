use super::{ScriptError, ScriptExecutor};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tscanner_config::ScriptRuleConfig;

const MAX_FILES_ARG_BYTES: usize = 128 * 1024;

impl ScriptExecutor {
    pub(super) fn spawn_command(
        &self,
        rule_config: &ScriptRuleConfig,
        workspace_root: &Path,
        files: &[String],
    ) -> Result<Vec<u8>, ScriptError> {
        let parts: Vec<&str> = rule_config.command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ScriptError::RunnerNotFound(rule_config.command.clone()));
        }

        let program = parts[0];
        let mut args: Vec<String> = parts
            .iter()
            .skip(1)
            .map(|part| (*part).to_string())
            .collect();
        args.push(workspace_root.to_string_lossy().to_string());

        let files_json = serde_json::to_string(files).map_err(|e| {
            ScriptError::InvalidOutput(format!("Failed to serialize script file list: {}", e))
        })?;
        if files_json.len() <= MAX_FILES_ARG_BYTES {
            args.push("--files".to_string());
            args.push(files_json);
        } else {
            (self.log_debug)(&format!(
                "Script command file list omitted from argv: {} bytes exceeds {}",
                files_json.len(),
                MAX_FILES_ARG_BYTES
            ));
        }
        if !rule_config.options.is_null() {
            let options_json = serde_json::to_string(&rule_config.options).map_err(|e| {
                ScriptError::InvalidOutput(format!("Failed to serialize script options: {}", e))
            })?;
            args.push("--options".to_string());
            args.push(options_json);
        }

        let mut child = Command::new(program)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(workspace_root)
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ScriptError::RunnerNotFound(program.to_string())
                } else {
                    ScriptError::IoError(e)
                }
            })?;
        let child_id = child.id();

        (self.log_debug)(&format!(
            "Script command pid={} started: {} {:?} (timeout={}s)",
            child_id, program, args, rule_config.timeout
        ));

        let stdout_handle = child.stdout.take().map(|mut stdout| {
            std::thread::spawn(move || {
                let mut output = Vec::new();
                let _ = stdout.read_to_end(&mut output);
                output
            })
        });
        let stderr_handle = child.stderr.take().map(|mut stderr| {
            std::thread::spawn(move || {
                let mut output = Vec::new();
                let _ = stderr.read_to_end(&mut output);
                output
            })
        });

        let timeout = if rule_config.timeout > 0 {
            Some(Duration::from_secs(rule_config.timeout))
        } else {
            None
        };
        let start = Instant::now();
        let mut next_wait_log = Duration::from_secs(5);

        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    let stdout = stdout_handle
                        .map(|handle| handle.join().unwrap_or_default())
                        .unwrap_or_default();
                    let stderr = stderr_handle
                        .map(|handle| handle.join().unwrap_or_default())
                        .unwrap_or_default();

                    (self.log_debug)(&format!(
                        "Script command pid={} exited after {}ms: status={}",
                        child_id,
                        start.elapsed().as_millis(),
                        status
                    ));

                    if !status.success() {
                        return Err(ScriptError::NonZeroExit {
                            code: status.code(),
                            stderr: String::from_utf8_lossy(&stderr).to_string(),
                        });
                    }

                    return Ok(stdout);
                }
                Ok(None) => {
                    if let Some(t) = timeout {
                        if start.elapsed() > t {
                            let _ = child.kill();
                            (self.log_debug)(&format!(
                                "Script command pid={} killed after timeout: {}s",
                                child_id, rule_config.timeout
                            ));
                            return Err(ScriptError::Timeout(rule_config.timeout));
                        }
                    }

                    if start.elapsed() >= next_wait_log {
                        (self.log_debug)(&format!(
                            "Script command pid={} still running after {}ms: program={} args={:?}",
                            child_id,
                            start.elapsed().as_millis(),
                            program,
                            args
                        ));
                        next_wait_log += Duration::from_secs(5);
                    }

                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) => return Err(ScriptError::IoError(e)),
            }
        }
    }
}
