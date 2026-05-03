use crate::bridge;
use crate::config::Config;
use crate::error::FreeCADError;
use crate::scripts::ScriptBuilder;
use super::{
    AddCircleInput, AddRectangleInput, CreateSketchInput, EdgeIndexResult, EdgeIndicesResult,
    ExtrudeInput, ObjectResult, RevolveAxis, RevolveInput, SketchPlane,
};

fn deserialize_object_result(val: serde_json::Value) -> Result<ObjectResult, FreeCADError> {
    if val.get("success").and_then(|v| v.as_bool()) == Some(false) {
        let msg = val["error"].as_str().unwrap_or("unknown error").to_string();
        return Err(FreeCADError::SubprocessFailed(msg));
    }
    serde_json::from_value(val).map_err(FreeCADError::JsonError)
}

fn deserialize_edge_indices(val: serde_json::Value) -> Result<EdgeIndicesResult, FreeCADError> {
    if val.get("success").and_then(|v| v.as_bool()) == Some(false) {
        let msg = val["error"].as_str().unwrap_or("unknown error").to_string();
        return Err(FreeCADError::SubprocessFailed(msg));
    }
    serde_json::from_value(val).map_err(FreeCADError::JsonError)
}

fn deserialize_edge_index(val: serde_json::Value) -> Result<EdgeIndexResult, FreeCADError> {
    if val.get("success").and_then(|v| v.as_bool()) == Some(false) {
        let msg = val["error"].as_str().unwrap_or("unknown error").to_string();
        return Err(FreeCADError::SubprocessFailed(msg));
    }
    serde_json::from_value(val).map_err(FreeCADError::JsonError)
}

fn plane_python_literal(p: SketchPlane) -> &'static str {
    match p {
        SketchPlane::XY => "XY",
        SketchPlane::XZ => "XZ",
        SketchPlane::YZ => "YZ",
    }
}

fn revolve_axis_literal(a: RevolveAxis) -> &'static str {
    match a {
        RevolveAxis::X => "X",
        RevolveAxis::Y => "Y",
        RevolveAxis::Z => "Z",
    }
}

/// Create a sketch on the given plane. Returns the sketch object_id.
pub fn create_sketch(input: &CreateSketchInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    let plane = plane_python_literal(input.plane);
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push("sketch = doc.addObject('Sketcher::SketchObject', 'Sketch')".to_string());
    b.push(format!("plane = {plane:?}"));
    b.push(
        "if plane == 'XY': sketch.Placement = FreeCAD.Placement()".to_string(),
    );
    b.push(
        "elif plane == 'XZ': sketch.Placement = FreeCAD.Placement(FreeCAD.Vector(0, 0, 0), FreeCAD.Rotation(FreeCAD.Vector(1, 0, 0), 90))".to_string(),
    );
    b.push(
        "elif plane == 'YZ': sketch.Placement = FreeCAD.Placement(FreeCAD.Vector(0, 0, 0), FreeCAD.Rotation(FreeCAD.Vector(0, 1, 0), 90))".to_string(),
    );
    b.push("doc.recompute()".to_string());
    b.push("doc.saveAs(DOC_PATH)".to_string());
    let script =
        b.finish(r#"{"success": True, "object_id": sketch.Name, "label": sketch.Label}"#);
    let val = bridge::run(script, config)?;
    deserialize_object_result(val)
}

/// Add a rectangle to an existing sketch. Returns edge indices.
pub fn add_rectangle(input: &AddRectangleInput, config: &Config) -> Result<EdgeIndicesResult, FreeCADError> {
    if input.width <= 0.0 || input.height <= 0.0 {
        return Err(FreeCADError::InvalidInput(
            "width and height must be positive".into(),
        ));
    }
    if !bridge::object_exists(&input.sketch_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.sketch_id.clone()));
    }

    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("sk = doc.getObject({:?})", input.sketch_id));
    b.push(format!("x, y, w, h = {}, {}, {}, {}", input.x, input.y, input.width, input.height));
    b.push(
        "g0 = Part.LineSegment(FreeCAD.Vector(x, y, 0), FreeCAD.Vector(x + w, y, 0))".to_string(),
    );
    b.push(
        "g1 = Part.LineSegment(FreeCAD.Vector(x + w, y, 0), FreeCAD.Vector(x + w, y + h, 0))"
            .to_string(),
    );
    b.push(
        "g2 = Part.LineSegment(FreeCAD.Vector(x + w, y + h, 0), FreeCAD.Vector(x, y + h, 0))"
            .to_string(),
    );
    b.push(
        "g3 = Part.LineSegment(FreeCAD.Vector(x, y + h, 0), FreeCAD.Vector(x, y, 0))".to_string(),
    );
    b.push("i0 = sk.addGeometry(g0, False)".to_string());
    b.push("i1 = sk.addGeometry(g1, False)".to_string());
    b.push("i2 = sk.addGeometry(g2, False)".to_string());
    b.push("i3 = sk.addGeometry(g3, False)".to_string());
    b.push("indices = [i0, i1, i2, i3]".to_string());
    b.push("doc.recompute()".to_string());
    b.push("doc.saveAs(DOC_PATH)".to_string());
    let script = b.finish(r#"{"success": True, "indices": indices}"#);
    let val = bridge::run(script, config)?;
    deserialize_edge_indices(val)
}

/// Add a circle to an existing sketch. Returns edge index.
pub fn add_circle(input: &AddCircleInput, config: &Config) -> Result<EdgeIndexResult, FreeCADError> {
    if input.radius <= 0.0 {
        return Err(FreeCADError::InvalidInput("radius must be positive".into()));
    }
    if !bridge::object_exists(&input.sketch_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.sketch_id.clone()));
    }

    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("sk = doc.getObject({:?})", input.sketch_id));
    b.push(format!(
        "circ = Part.Circle(FreeCAD.Vector({}, {}, 0), FreeCAD.Vector(0, 0, 1), {})",
        input.cx, input.cy, input.radius
    ));
    b.push("idx = sk.addGeometry(circ, False)".to_string());
    b.push("doc.recompute()".to_string());
    b.push("doc.saveAs(DOC_PATH)".to_string());
    let script = b.finish(r#"{"success": True, "index": idx}"#);
    let val = bridge::run(script, config)?;
    deserialize_edge_index(val)
}

/// Extrude a sketch profile along its plane normal into a solid (`Part::Feature`).
pub fn extrude(input: &ExtrudeInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    if input.depth <= 0.0 {
        return Err(FreeCADError::InvalidInput("depth must be positive".into()));
    }
    if !bridge::object_exists(&input.sketch_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.sketch_id.clone()));
    }

    let symmetric = input.symmetric.unwrap_or(false);
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("sk = doc.getObject({:?})", input.sketch_id));
    b.push(format!("depth = {}", input.depth));
    b.push(format!("symmetric = {}", if symmetric { "True" } else { "False" }));
    b.push("doc.recompute()".to_string());
    b.push(
        "wires = sk.Shape.Wires if hasattr(sk.Shape, 'Wires') else []".to_string(),
    );
    b.push(
        "if len(wires) < 1: raise RuntimeError('sketch has no closed wires to extrude')".to_string(),
    );
    b.push("wire = wires[0]".to_string());
    b.push("face = Part.Face(wire)".to_string());
    b.push(
        "n = sk.Placement.Rotation.multVec(FreeCAD.Vector(0, 0, 1))".to_string(),
    );
    b.push(
        "if symmetric: face = face.translated(FreeCAD.Vector(-n.x * depth / 2, -n.y * depth / 2, -n.z * depth / 2))"
            .to_string(),
    );
    b.push(
        "ext = FreeCAD.Vector(n.x * depth, n.y * depth, n.z * depth)".to_string(),
    );
    b.push("solid = face.extrude(ext)".to_string());
    b.push("obj = doc.addObject('Part::Feature', 'Extrusion')".to_string());
    b.push("obj.Shape = solid".to_string());
    b.push("doc.recompute()".to_string());
    b.push("doc.saveAs(DOC_PATH)".to_string());
    let script = b.finish(r#"{"success": True, "object_id": obj.Name, "label": obj.Label}"#);
    let val = bridge::run(script, config)?;
    deserialize_object_result(val)
}

/// Revolve a sketch profile around an axis through the sketch origin (`Part::Feature`).
pub fn revolve(input: &RevolveInput, config: &Config) -> Result<ObjectResult, FreeCADError> {
    if input.angle_deg <= 0.0 {
        return Err(FreeCADError::InvalidInput(
            "angle_deg must be positive".into(),
        ));
    }
    if !bridge::object_exists(&input.sketch_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.sketch_id.clone()));
    }

    let axis = revolve_axis_literal(input.axis);
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("sk = doc.getObject({:?})", input.sketch_id));
    b.push(format!("angle_deg = {}", input.angle_deg));
    b.push(format!("axis_key = {axis:?}"));
    b.push("doc.recompute()".to_string());
    b.push(
        "wires = sk.Shape.Wires if hasattr(sk.Shape, 'Wires') else []".to_string(),
    );
    b.push(
        "if len(wires) < 1: raise RuntimeError('sketch has no closed wires to revolve')".to_string(),
    );
    b.push("wire = wires[0]".to_string());
    b.push("face = Part.Face(wire)".to_string());
    b.push("origin = sk.Placement.Base".to_string());
    b.push(
        "axis_map = {'X': FreeCAD.Vector(1, 0, 0), 'Y': FreeCAD.Vector(0, 1, 0), 'Z': FreeCAD.Vector(0, 0, 1)}"
            .to_string(),
    );
    b.push("axis_local = axis_map[axis_key]".to_string());
    b.push(
        "dir_global = sk.Placement.Rotation.multVec(axis_local)".to_string(),
    );
    b.push("solid = face.revolve(origin, dir_global, angle_deg)".to_string());
    b.push("obj = doc.addObject('Part::Feature', 'Revolve')".to_string());
    b.push("obj.Shape = solid".to_string());
    b.push("doc.recompute()".to_string());
    b.push("doc.saveAs(DOC_PATH)".to_string());
    let script = b.finish(r#"{"success": True, "object_id": obj.Name, "label": obj.Label}"#);
    let val = bridge::run(script, config)?;
    deserialize_object_result(val)
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
    fn add_rectangle_rejects_non_positive_size() {
        let input = AddRectangleInput {
            sketch_id: "Sketch001".into(),
            x: 0.0,
            y: 0.0,
            width: -1.0,
            height: 10.0,
        };
        assert!(matches!(
            add_rectangle(&input, &fixture_config()),
            Err(FreeCADError::InvalidInput(_))
        ));
    }

    #[test]
    fn add_circle_rejects_zero_radius() {
        let input = AddCircleInput {
            sketch_id: "Sketch001".into(),
            cx: 0.0,
            cy: 0.0,
            radius: 0.0,
        };
        assert!(matches!(
            add_circle(&input, &fixture_config()),
            Err(FreeCADError::InvalidInput(_))
        ));
    }

    #[test]
    fn extrude_rejects_non_positive_depth() {
        let input = ExtrudeInput {
            sketch_id: "Sketch001".into(),
            depth: 0.0,
            symmetric: None,
        };
        assert!(matches!(
            extrude(&input, &fixture_config()),
            Err(FreeCADError::InvalidInput(_))
        ));
    }

    #[test]
    fn revolve_rejects_zero_angle() {
        let input = RevolveInput {
            sketch_id: "Sketch001".into(),
            axis: RevolveAxis::Z,
            angle_deg: 0.0,
        };
        assert!(matches!(
            revolve(&input, &fixture_config()),
            Err(FreeCADError::InvalidInput(_))
        ));
    }

    #[test]
    fn create_sketch_returns_err_when_binary_missing() {
        let input = CreateSketchInput {
            plane: SketchPlane::XY,
        };
        assert!(create_sketch(&input, &fixture_config()).is_err());
    }
}
