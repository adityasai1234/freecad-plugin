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
    let _ = (script, config);
    todo!()
}
