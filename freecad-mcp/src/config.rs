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
        which::which(&self.freecadcmd_path).map_err(|_| {
            anyhow::anyhow!("FreeCADCmd not found at {:?}", self.freecadcmd_path)
        })?;
        std::fs::create_dir_all(&self.work_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    // Serialize all env-mutating tests to avoid cross-thread contamination.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn defaults_are_applied() {
        let _g = ENV_LOCK.lock().unwrap();
        env::remove_var("FREECADCMD_PATH");
        env::remove_var("FREECAD_WORK_DIR");
        env::remove_var("FREECAD_TIMEOUT");
        env::remove_var("FREECAD_GUI");
        let cfg = Config::from_env().unwrap();
        assert_eq!(cfg.freecadcmd_path, PathBuf::from("/usr/bin/FreeCADCmd"));
        assert_eq!(cfg.work_dir, PathBuf::from("/tmp/freecad_workspace"));
        assert_eq!(cfg.session_doc, PathBuf::from("/tmp/freecad_workspace/session.FCStd"));
        assert_eq!(cfg.timeout_secs, 30);
        assert!(!cfg.gui_mode);
    }

    #[test]
    fn env_vars_override_defaults() {
        let _g = ENV_LOCK.lock().unwrap();
        env::set_var("FREECADCMD_PATH", "/custom/FreeCADCmd");
        env::set_var("FREECAD_WORK_DIR", "/custom/workspace");
        env::set_var("FREECAD_TIMEOUT", "60");
        env::set_var("FREECAD_GUI", "true");
        let cfg = Config::from_env().unwrap();
        assert_eq!(cfg.freecadcmd_path, PathBuf::from("/custom/FreeCADCmd"));
        assert_eq!(cfg.work_dir, PathBuf::from("/custom/workspace"));
        assert_eq!(cfg.timeout_secs, 60);
        assert!(cfg.gui_mode);
        env::remove_var("FREECADCMD_PATH");
        env::remove_var("FREECAD_WORK_DIR");
        env::remove_var("FREECAD_TIMEOUT");
        env::remove_var("FREECAD_GUI");
    }

    #[test]
    fn validate_fails_for_missing_binary() {
        let _g = ENV_LOCK.lock().unwrap();
        env::set_var("FREECADCMD_PATH", "/nonexistent/FreeCADCmd");
        let cfg = Config::from_env().unwrap();
        assert!(cfg.validate().is_err());
        env::remove_var("FREECADCMD_PATH");
    }
}
