use super::{ScriptError, ScriptExecutor};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tscanner_config::ScriptRuleConfig;

impl ScriptExecutor {
    pub(super) fn spawn_command(
        &self,
        rule_config: &ScriptRuleConfig,
        input: &[u8],
    ) -> Result<Vec<u8>, ScriptError> {
        let parts: Vec<&str> = rule_config.command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ScriptError::RunnerNotFound(rule_config.command.clone()));
        }

        let program = parts[0];
        let args = &parts[1..];

        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&self.config_dir)
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
            "Script command pid={} started: {} {:?} (stdin={} bytes, timeout={}s)",
            child_id,
            program,
            args,
            input.len(),
            rule_config.timeout
        ));

        let mut stdin = child.stdin.take().unwrap();
        let input_clone = input.to_vec();
        let write_handle = std::thread::spawn(move || stdin.write_all(&input_clone));
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
                    let _ = write_handle.join();

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
