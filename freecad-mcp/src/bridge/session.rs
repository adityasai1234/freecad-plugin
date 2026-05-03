use std::path::PathBuf;

use crate::config::Config;
use crate::error::FreeCADError;

/// Create the workspace directory if it does not already exist.
pub fn ensure_workspace(config: &Config) -> Result<(), FreeCADError> {
    let _ = config;
    todo!()
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
