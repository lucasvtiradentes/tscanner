use super::types::AiError;
use super::AiExecutor;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

impl AiExecutor {
    pub(super) fn spawn_ai_command(
        &self,
        program: &str,
        args: &[String],
        prompt: &str,
        timeout_ms: u64,
        cancelled: &Arc<AtomicBool>,
    ) -> Result<String, AiError> {
        (self.log_debug)(&format!(
            "Spawning: {} {:?} (cwd: {:?})",
            program, args, self.workspace_root
        ));

        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&self.workspace_root)
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    AiError::ProviderNotFound(program.to_string())
                } else {
                    AiError::IoError(e)
                }
            })?;

        let mut stdin = child.stdin.take().unwrap();
        let prompt_clone = prompt.to_string();
        let write_handle = std::thread::spawn(move || stdin.write_all(prompt_clone.as_bytes()));
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

        let timeout = if timeout_ms > 0 {
            Some(Duration::from_millis(timeout_ms))
        } else {
            None
        };
        let start = Instant::now();

        loop {
            if cancelled.load(Ordering::SeqCst) {
                let _ = child.kill();
                return Ok(String::new());
            }

            match child.try_wait() {
                Ok(Some(status)) => {
                    let _ = write_handle.join();

                    let stdout = stdout_handle
                        .map(|handle| handle.join().unwrap_or_default())
                        .unwrap_or_default();
                    let stderr = stderr_handle
                        .map(|handle| handle.join().unwrap_or_default())
                        .unwrap_or_default();

                    if !status.success() {
                        let stderr_str = String::from_utf8_lossy(&stderr).to_string();
                        let stdout_str = String::from_utf8_lossy(&stdout).to_string();
                        let error_output = if stderr_str.is_empty() {
                            stdout_str
                        } else {
                            stderr_str
                        };
                        return Err(AiError::NonZeroExit {
                            code: status.code(),
                            stderr: error_output,
                        });
                    }

                    return Ok(String::from_utf8_lossy(&stdout).to_string());
                }
                Ok(None) => {
                    if let Some(t) = timeout {
                        if start.elapsed() > t {
                            let _ = child.kill();
                            return Err(AiError::Timeout(timeout_ms / 1000));
                        }
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => return Err(AiError::IoError(e)),
            }
        }
    }
}
