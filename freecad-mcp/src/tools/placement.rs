use crate::bridge;
use crate::config::Config;
use crate::error::FreeCADError;
use crate::scripts::ScriptBuilder;
use super::{ObjectResult, PlaceObjectInput, RotateObjectInput};

/// Move an object to the given x/y/z position (and optionally set yaw/pitch/roll).
pub fn place_object(input: &PlaceObjectInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    if !bridge::object_exists(&input.object_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.object_id.clone()));
    }
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("obj = doc.getObject({id:?})", id = input.object_id));
    b.push(format!(
        "obj.Placement.Base = FreeCAD.Vector({}, {}, {})",
        input.x, input.y, input.z
    ));
    if let (Some(yaw), Some(pitch), Some(roll)) = (input.yaw, input.pitch, input.roll) {
        b.push(format!(
            "obj.Placement.Rotation = FreeCAD.Rotation({yaw}, {pitch}, {roll})"
        ));
    }
    b.push("doc.recompute()");
    b.push("doc.saveAs(DOC_PATH)");
    let script = b.finish(r#"{"success": True, "object_id": obj.Name, "label": obj.Label}"#);
    let val = bridge::run(script, config)?;
    deserialize_object_result(val)
}

/// Rotate an object to yaw/pitch/roll angles (degrees).
pub fn rotate_object(input: &RotateObjectInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    if !bridge::object_exists(&input.object_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.object_id.clone()));
    }
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("obj = doc.getObject({id:?})", id = input.object_id));
    b.push(format!(
        "obj.Placement.Rotation = FreeCAD.Rotation({}, {}, {})",
        input.yaw, input.pitch, input.roll
    ));
    b.push("doc.recompute()");
    b.push("doc.saveAs(DOC_PATH)");
    let script = b.finish(r#"{"success": True, "object_id": obj.Name, "label": obj.Label}"#);
    let val = bridge::run(script, config)?;
    deserialize_object_result(val)
}

fn deserialize_object_result(val: serde_json::Value) -> Result<ObjectResult, FreeCADError> {
    if val.get("success").and_then(|v| v.as_bool()) == Some(false) {
        let msg = val["error"].as_str().unwrap_or("unknown error").to_string();
        return Err(FreeCADError::SubprocessFailed(msg));
    }
    serde_json::from_value(val).map_err(FreeCADError::JsonError)
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
    fn place_object_returns_not_found_when_binary_missing() {
        // With a nonexistent FreeCADCmd the bridge will fail before ObjectNotFound,
        // but we verify it returns an Err (not a panic).
        let input = PlaceObjectInput {
            object_id: "Box001".into(),
            x: 0.0, y: 0.0, z: 0.0,
            yaw: None, pitch: None, roll: None,
        };
        assert!(place_object(&input, &fixture_config()).is_err());
    }

    #[test]
    fn rotate_object_returns_err_when_binary_missing() {
        let input = RotateObjectInput {
            object_id: "Box001".into(),
            yaw: 0.0, pitch: 0.0, roll: 90.0,
        };
        assert!(rotate_object(&input, &fixture_config()).is_err());
    }
}
