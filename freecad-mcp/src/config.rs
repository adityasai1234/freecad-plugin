use std::path::PathBuf;

/// Runtime configuration loaded from environment variables.
pub struct Config {
    pub freecadcmd_path: PathBuf,
    pub work_dir: PathBuf,
    /// Computed as `work_dir/session.FCStd`.
    pub session_doc: PathBuf,
    pub timeout_secs: u64,
    pub gui_mode: bool,
}

impl Config {
    /// Build a `Config` from environment variables, falling back to safe defaults.
    pub fn from_env() -> anyhow::Result<Config> {
        let freecadcmd_path = PathBuf::from(
            std::env::var("FREECADCMD_PATH").unwrap_or_else(|_| "/usr/bin/FreeCADCmd".into()),
        );
        let work_dir = PathBuf::from(
            std::env::var("FREECAD_WORK_DIR")
                .unwrap_or_else(|_| "/tmp/freecad_workspace".into()),
        );
        let session_doc = work_dir.join("session.FCStd");
        let timeout_secs = std::env::var("FREECAD_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30u64);
        let gui_mode = std::env::var("FREECAD_GUI")
            .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);

        Ok(Config { freecadcmd_path, work_dir, session_doc, timeout_secs, gui_mode })
    }

    /// Validate that the binary exists and the work directory can be created.
    pub fn validate(&self) -> anyhow::Result<()> {
        todo!()
    }
}
