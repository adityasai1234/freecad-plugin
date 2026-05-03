use crate::bridge;
use crate::config::Config;
use crate::error::FreeCADError;
use crate::scripts::ScriptBuilder;
use super::{BooleanInput, ObjectResult};

/// Fuse two objects (Part::Fuse). Returns the resulting object_id.
pub fn boolean_union(input: &BooleanInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    check_both(input, config)?;
    let script = script_for_boolean("Part::Fuse", &input.base_id, &input.tool_id,
        input.label.as_deref().unwrap_or("Fusion"), config);
    let val = bridge::run(script, config)?;
    deserialize(val)
}

/// Subtract tool from base (Part::Cut). Returns the resulting object_id.
pub fn boolean_cut(input: &BooleanInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    check_both(input, config)?;
    let script = script_for_boolean("Part::Cut", &input.base_id, &input.tool_id,
        input.label.as_deref().unwrap_or("Cut"), config);
    let val = bridge::run(script, config)?;
    deserialize(val)
}

/// Intersect two objects (Part::Common). Returns the resulting object_id.
pub fn boolean_intersection(input: &BooleanInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    check_both(input, config)?;
    let script = script_for_boolean("Part::Common", &input.base_id, &input.tool_id,
        input.label.as_deref().unwrap_or("Common"), config);
    let val = bridge::run(script, config)?;
    deserialize(val)
}

fn check_both(input: &BooleanInput, config: &Config) -> Result<(), FreeCADError> {
    if !bridge::object_exists(&input.base_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.base_id.clone()));
    }
    if !bridge::object_exists(&input.tool_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.tool_id.clone()));
    }
    Ok(())
}

fn script_for_boolean(op_type: &str, base: &str, tool: &str, label: &str, config: &Config) -> String {
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("base_obj = doc.getObject({base:?})"));
    b.push(format!("tool_obj = doc.getObject({tool:?})"));
    b.push(format!("obj = doc.addObject({op_type:?}, {label:?})"));
    b.push("obj.Base = base_obj");
    b.push("obj.Tool = tool_obj");
    b.push("doc.recompute()");
    b.push("doc.saveAs(DOC_PATH)");
    b.finish(r#"{"success": True, "object_id": obj.Name, "label": obj.Label}"#)
}

fn deserialize(val: serde_json::Value) -> Result<ObjectResult, FreeCADError> {
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

    fn input(base: &str, tool: &str) -> BooleanInput {
        BooleanInput { base_id: base.into(), tool_id: tool.into(), label: None }
    }

    #[test]
    fn union_returns_err_when_binary_missing() {
        assert!(boolean_union(&input("Box001", "Cylinder001"), &fixture_config()).is_err());
    }

    #[test]
    fn cut_returns_err_when_binary_missing() {
        assert!(boolean_cut(&input("Box001", "Cylinder001"), &fixture_config()).is_err());
    }

    #[test]
    fn intersection_returns_err_when_binary_missing() {
        assert!(boolean_intersection(&input("Box001", "Sphere001"), &fixture_config()).is_err());
    }
}
