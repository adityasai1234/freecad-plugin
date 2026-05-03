use crate::bridge;
use crate::config::Config;
use crate::error::FreeCADError;
use crate::scripts::ScriptBuilder;
use super::{CreateBoxInput, CreateCylinderInput, CreateSphereInput, CreateConeInput, ObjectResult};

/// Create a box primitive in the session document.
pub fn create_box(input: &CreateBoxInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    input.validate()?;
    let label = input.label.as_deref().unwrap_or("Box");
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("obj = doc.addObject('Part::Box', {label:?})"));
    b.push(format!("obj.Length = {}", input.length));
    b.push(format!("obj.Width  = {}", input.width));
    b.push(format!("obj.Height = {}", input.height));
    b.push("doc.recompute()");
    b.push("doc.saveAs(DOC_PATH)");
    let script = b.finish(r#"{"success": True, "object_id": obj.Name, "label": obj.Label}"#);
    let val = bridge::run(script, config)?;
    deserialize_object_result(val)
}

/// Create a cylinder primitive in the session document.
pub fn create_cylinder(input: &CreateCylinderInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    input.validate()?;
    let label = input.label.as_deref().unwrap_or("Cylinder");
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("obj = doc.addObject('Part::Cylinder', {label:?})"));
    b.push(format!("obj.Radius = {}", input.radius));
    b.push(format!("obj.Height = {}", input.height));
    b.push("doc.recompute()");
    b.push("doc.saveAs(DOC_PATH)");
    let script = b.finish(r#"{"success": True, "object_id": obj.Name, "label": obj.Label}"#);
    let val = bridge::run(script, config)?;
    deserialize_object_result(val)
}

/// Create a sphere primitive in the session document.
pub fn create_sphere(input: &CreateSphereInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    input.validate()?;
    let label = input.label.as_deref().unwrap_or("Sphere");
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("obj = doc.addObject('Part::Sphere', {label:?})"));
    b.push(format!("obj.Radius = {}", input.radius));
    b.push("doc.recompute()");
    b.push("doc.saveAs(DOC_PATH)");
    let script = b.finish(r#"{"success": True, "object_id": obj.Name, "label": obj.Label}"#);
    let val = bridge::run(script, config)?;
    deserialize_object_result(val)
}

/// Create a cone primitive in the session document.
pub fn create_cone(input: &CreateConeInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    input.validate()?;
    let label = input.label.as_deref().unwrap_or("Cone");
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("obj = doc.addObject('Part::Cone', {label:?})"));
    b.push(format!("obj.Radius1 = {}", input.radius1));
    b.push(format!("obj.Radius2 = {}", input.radius2));
    b.push(format!("obj.Height  = {}", input.height));
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
    fn create_box_rejects_zero_dimensions() {
        let input = CreateBoxInput { length: 0.0, width: 10.0, height: 10.0, label: None };
        assert!(matches!(
            create_box(&input, &fixture_config()),
            Err(FreeCADError::InvalidInput(_))
        ));
    }

    #[test]
    fn create_cylinder_rejects_negative_radius() {
        let input = CreateCylinderInput { radius: -1.0, height: 10.0, label: None };
        assert!(matches!(
            create_cylinder(&input, &fixture_config()),
            Err(FreeCADError::InvalidInput(_))
        ));
    }

    #[test]
    fn create_sphere_rejects_zero_radius() {
        let input = CreateSphereInput { radius: 0.0, label: None };
        assert!(matches!(
            create_sphere(&input, &fixture_config()),
            Err(FreeCADError::InvalidInput(_))
        ));
    }

    #[test]
    fn create_cone_rejects_negative_height() {
        let input = CreateConeInput { radius1: 5.0, radius2: 3.0, height: -1.0, label: None };
        assert!(matches!(
            create_cone(&input, &fixture_config()),
            Err(FreeCADError::InvalidInput(_))
        ));
    }
}
