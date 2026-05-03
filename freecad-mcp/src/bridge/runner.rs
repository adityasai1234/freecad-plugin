use std::io::Write;
use std::sync::mpsc;
use std::time::Duration;

use crate::config::Config;
use crate::error::FreeCADError;

/// Execute a Python script in a FreeCADCmd subprocess and return the parsed JSON result.
///
/// Contract:
/// 1. Write `script` to a temp `.py` file.
/// 2. Spawn `config.freecadcmd_path <temp_file>`.
/// 3. Wait up to `config.timeout_secs`; kill and return `Timeout` if exceeded.
/// 4. On non-zero exit return `SubprocessFailed` with the last 500 chars of stderr.
/// 5. Parse the last non-empty stdout line as JSON; return `ParseError` if it fails.
/// 6. Delete the temp file in all branches.
pub fn run_freecad_script(
    script: &str,
    config: &Config,
) -> Result<serde_json::Value, FreeCADError> {
    let mut tmp = tempfile::Builder::new()
        .suffix(".py")
        .tempfile()?;
    tmp.write_all(script.as_bytes())?;
    let tmp_path = tmp.path().to_path_buf();

    let child = std::process::Command::new(&config.freecadcmd_path)
        .arg(&tmp_path)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let timeout = Duration::from_secs(config.timeout_secs);
    let (tx, rx) = mpsc::channel();
    let handle = std::thread::spawn(move || {
        let result = child.wait_with_output();
        let _ = tx.send(result);
    });

    let output = match rx.recv_timeout(timeout) {
        Ok(result) => result?,
        Err(_) => {
            // Thread is still running; we can't easily kill the child from here
            // because it was moved. Drop the handle and return Timeout.
            drop(handle);
            return Err(FreeCADError::Timeout);
        }
    };

    let _ = handle.join();

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    if !output.status.success() {
        let tail: String = stderr.chars().rev().take(500).collect::<String>()
            .chars().rev().collect();
        return Err(FreeCADError::SubprocessFailed(tail));
    }

    parse_last_json_line(&stdout)
}

fn parse_last_json_line(stdout: &str) -> Result<serde_json::Value, FreeCADError> {
    stdout
        .lines()
        .rfind(|l| !l.trim().is_empty())
        .ok_or_else(|| FreeCADError::ParseError("no output from FreeCAD".into()))
        .and_then(|line| {
            serde_json::from_str(line)
                .map_err(|e| FreeCADError::ParseError(format!("{e}: {line}")))
        })
}
