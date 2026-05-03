use std::path::PathBuf;

use crate::config::Config;
use crate::error::FreeCADError;

/// Create the workspace directory if it does not already exist. Idempotent.
pub fn ensure_workspace(config: &Config) -> Result<(), FreeCADError> {
    std::fs::create_dir_all(&config.work_dir)?;
    Ok(())
}

/// Return the path of the persistent session document.
pub fn session_doc_path(config: &Config) -> PathBuf {
    config.session_doc.clone()
}

/// Check whether an object with the given ID exists in the session document.
pub fn object_exists(object_id: &str, config: &Config) -> Result<bool, FreeCADError> {
    let _ = (object_id, config);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(work_dir: std::path::PathBuf) -> Config {
        let session_doc = work_dir.join("session.FCStd");
        Config {
            freecadcmd_path: std::path::PathBuf::from("/usr/bin/FreeCADCmd"),
            work_dir,
            session_doc,
            timeout_secs: 5,
            gui_mode: false,
        }
    }

    #[test]
    fn creates_directory_when_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let work_dir = tmp.path().join("new_workspace");
        let cfg = test_config(work_dir.clone());
        assert!(!work_dir.exists());
        ensure_workspace(&cfg).unwrap();
        assert!(work_dir.exists());
    }

    #[test]
    fn idempotent_when_directory_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let cfg = test_config(tmp.path().to_path_buf());
        ensure_workspace(&cfg).unwrap();
        ensure_workspace(&cfg).unwrap(); // second call must not error
    }
}
