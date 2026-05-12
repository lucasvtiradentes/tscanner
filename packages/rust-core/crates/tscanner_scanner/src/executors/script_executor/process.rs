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

        (self.log_debug)(&format!(
            "Running command: {} {:?} (cwd: {:?})",
            program, args, self.config_dir
        ));

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

        let mut stdin = child.stdin.take().unwrap();
        let input_clone = input.to_vec();
        let write_handle = std::thread::spawn(move || stdin.write_all(&input_clone));

        let timeout = if rule_config.timeout > 0 {
            Some(Duration::from_secs(rule_config.timeout))
        } else {
            None
        };
        let start = Instant::now();

        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    let _ = write_handle.join();

                    let mut stdout = Vec::new();
                    let mut stderr = Vec::new();

                    if let Some(mut stdout_handle) = child.stdout.take() {
                        let _ = stdout_handle.read_to_end(&mut stdout);
                    }
                    if let Some(mut stderr_handle) = child.stderr.take() {
                        let _ = stderr_handle.read_to_end(&mut stderr);
                    }

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
                            return Err(ScriptError::Timeout(rule_config.timeout));
                        }
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) => return Err(ScriptError::IoError(e)),
            }
        }
    }
}
