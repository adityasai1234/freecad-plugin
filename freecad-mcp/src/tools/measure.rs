use crate::bridge;
use crate::config::Config;
use crate::error::FreeCADError;
use crate::scripts::ScriptBuilder;
use super::{DistanceResult, MeasureInput};

/// Return the minimum distance in mm between two objects.
pub fn measure_distance(input: &MeasureInput, config: &Config) -> Result<DistanceResult, FreeCADError> {
    if !bridge::object_exists(&input.object_id_a, config)? {
        return Err(FreeCADError::ObjectNotFound(input.object_id_a.clone()));
    }
    if !bridge::object_exists(&input.object_id_b, config)? {
        return Err(FreeCADError::ObjectNotFound(input.object_id_b.clone()));
    }

    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("a = doc.getObject({:?})", input.object_id_a));
    b.push(format!("b = doc.getObject({:?})", input.object_id_b));
    b.push("doc.recompute()".to_string());
    b.push("dlist, _, _ = a.Shape.distToShape(b.Shape)".to_string());
    b.push(
        "_mindist = min([float(x) for x in dlist], default=0.0) if dlist else 0.0".to_string(),
    );
    let script = b.finish(r#"{"success": True, "distance_mm": float(_mindist)}"#);
    let val = bridge::run(script, config)?;
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
    fn measure_returns_err_when_binary_missing() {
        let input = MeasureInput {
            object_id_a: "Box001".into(),
            object_id_b: "Box002".into(),
        };
        assert!(measure_distance(&input, &fixture_config()).is_err());
    }
}
