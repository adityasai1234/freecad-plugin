use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::bridge;
use crate::config::Config;
use crate::error::FreeCADError;
use crate::scripts::ScriptBuilder;
use super::{ExportInput, ExportResult, SaveResult};

fn escape_py_path(p: &Path) -> String {
    p.display().to_string().replace('\\', "\\\\")
}

fn ensure_export_path(
    config: &Config,
    object_id: &str,
    filename: Option<&str>,
    ext: &str,
) -> Result<PathBuf, FreeCADError> {
    bridge::ensure_workspace(config)?;
    let ext_lc = ext.to_ascii_lowercase();
    let name = match filename {
        None => {
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            format!("{}_{}.{}", object_id, ts, ext_lc)
        }
        Some(raw) => {
            let base = Path::new(raw)
                .file_name()
                .and_then(|s| s.to_str())
                .filter(|s| !s.is_empty() && *s != "." && *s != "..")
                .ok_or_else(|| FreeCADError::InvalidInput("invalid export filename".into()))?;
            let pb = Path::new(base);
            match pb.extension().and_then(|e| e.to_str()) {
                Some(e) if extension_ok(e, &ext_lc) => base.to_string(),
                Some(_) => {
                    return Err(FreeCADError::InvalidInput(format!(
                        "filename extension must match .{}",
                        ext_lc
                    )));
                }
                None => format!(
                    "{}.{}",
                    base,
                    if ext_lc == "step" { "step" } else { &ext_lc }
                ),
            }
        }
    };
    Ok(config.work_dir.join(name))
}

fn deserialize_export(val: serde_json::Value) -> Result<ExportResult, FreeCADError> {
    if val.get("success").and_then(|v| v.as_bool()) == Some(false) {
        let msg = val["error"].as_str().unwrap_or("unknown error").to_string();
        return Err(FreeCADError::SubprocessFailed(msg));
    }
    serde_json::from_value(val).map_err(FreeCADError::JsonError)
}

fn extension_ok(file_ext: &str, expected: &str) -> bool {
    let fe = file_ext.to_ascii_lowercase();
    match expected {
        "step" => fe == "step" || fe == "stp",
        other => fe == other,
    }
}

fn script_export_mesh_template(input: &ExportInput, config: &Config, out_path: &Path) -> String {
    let out_esc = escape_py_path(out_path);
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push("import Mesh".to_string());
    b.push(format!("obj = doc.getObject({id:?})", id = input.object_id));
    b.push("doc.recompute()".to_string());
    b.push("mesh = Mesh.Mesh(obj.Shape.tessellate(0.05))".to_string());
    b.push(format!("OUT_PATH = r\"{out_esc}\""));
    b.push("mesh.write(OUT_PATH)".to_string());
    b.push("_sz = os.path.getsize(OUT_PATH)".to_string());
    b.push("doc.saveAs(DOC_PATH)".to_string());
    b.finish(r#"{"success": True, "path": OUT_PATH, "size_bytes": _sz}"#)
}

/// Save the session document in place.
pub fn save_document(config: &Config) -> Result<SaveResult, FreeCADError> {
    bridge::ensure_workspace(config)?;
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push("doc.saveAs(DOC_PATH)".to_string());
    let script = b.finish(r#"{"success": True, "path": DOC_PATH}"#);
    let val = bridge::run(script, config)?;
    if val.get("success").and_then(|v| v.as_bool()) == Some(false) {
        let msg = val["error"].as_str().unwrap_or("unknown error").to_string();
        return Err(FreeCADError::SubprocessFailed(msg));
    }
    serde_json::from_value(val).map_err(FreeCADError::JsonError)
}

/// Export an object to STL. Returns file path and size in bytes.
pub fn export_stl(input: &ExportInput, config: &Config) -> Result<ExportResult, FreeCADError> {
    if !bridge::object_exists(&input.object_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.object_id.clone()));
    }
    let out_path = ensure_export_path(config, &input.object_id, input.filename.as_deref(), "stl")?;
    let script = script_export_mesh_template(input, config, &out_path);
    deserialize_export(bridge::run(script, config)?)
}

/// Export an object to STEP. Returns file path and size in bytes.
pub fn export_step(input: &ExportInput, config: &Config) -> Result<ExportResult, FreeCADError> {
    if !bridge::object_exists(&input.object_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.object_id.clone()));
    }
    let out_path = ensure_export_path(config, &input.object_id, input.filename.as_deref(), "step")?;
    let out_esc = escape_py_path(&out_path);
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("obj = doc.getObject({id:?})", id = input.object_id));
    b.push("doc.recompute()".to_string());
    b.push(format!("OUT_PATH = r\"{out_esc}\""));
    b.push("obj.Shape.exportStep(OUT_PATH)".to_string());
    b.push("_sz = os.path.getsize(OUT_PATH)".to_string());
    b.push("doc.saveAs(DOC_PATH)".to_string());
    let script = b.finish(r#"{"success": True, "path": OUT_PATH, "size_bytes": _sz}"#);
    deserialize_export(bridge::run(script, config)?)
}

/// Export an object to OBJ. Returns file path and size in bytes.
pub fn export_obj(input: &ExportInput, config: &Config) -> Result<ExportResult, FreeCADError> {
    if !bridge::object_exists(&input.object_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.object_id.clone()));
    }
    let out_path = ensure_export_path(config, &input.object_id, input.filename.as_deref(), "obj")?;
    let script = script_export_mesh_template(input, config, &out_path);
    deserialize_export(bridge::run(script, config)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_config() -> Config {
        let work_dir = PathBuf::from("/tmp/freecad_test");
        Config {
            freecadcmd_path: PathBuf::from("/nonexistent/FreeCADCmd"),
            session_doc: work_dir.join("session.FCStd"),
            work_dir,
            timeout_secs: 5,
            gui_mode: false,
        }
    }

    #[test]
    fn save_document_returns_err_when_binary_missing() {
        assert!(save_document(&fixture_config()).is_err());
    }

    #[test]
    fn export_stl_returns_err_when_binary_missing() {
        let input = ExportInput {
            object_id: "Box001".into(),
            filename: None,
        };
        assert!(export_stl(&input, &fixture_config()).is_err());
    }

    #[test]
    fn export_step_returns_err_when_binary_missing() {
        let input = ExportInput {
            object_id: "Box001".into(),
            filename: None,
        };
        assert!(export_step(&input, &fixture_config()).is_err());
    }

    #[test]
    fn export_obj_returns_err_when_binary_missing() {
        let input = ExportInput {
            object_id: "Box001".into(),
            filename: None,
        };
        assert!(export_obj(&input, &fixture_config()).is_err());
    }

    #[test]
    fn export_rejects_wrong_extension() {
        let cfg = fixture_config();
        let input = ExportInput {
            object_id: "Box001".into(),
            filename: Some("out.step".into()),
        };
        assert!(matches!(
            ensure_export_path(&cfg, "Box001", input.filename.as_deref(), "stl"),
            Err(FreeCADError::InvalidInput(_))
        ));
    }
}
