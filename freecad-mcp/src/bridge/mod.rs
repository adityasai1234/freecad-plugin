pub mod runner;
pub mod session;

pub use runner::run_freecad_script;
pub use session::{ensure_workspace, object_exists, session_doc_path};

use crate::config::Config;
use crate::error::FreeCADError;

/// Run a pre-built script string and return the parsed JSON result.
pub fn run(script: String, config: &Config) -> Result<serde_json::Value, FreeCADError> {
    run_freecad_script(&script, config)
}
