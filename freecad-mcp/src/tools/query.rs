use crate::bridge;
use crate::config::Config;
use crate::error::FreeCADError;
use crate::scripts::ScriptBuilder;
use super::{InfoResult, ListResult, ObjectIdInput, StatsResult};

/// List all Part objects in the session document.
pub fn list_objects(config: &Config) -> Result<ListResult, FreeCADError> {
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push("objects = [");
    b.push("    {\"id\": o.Name, \"label\": o.Label, \"type_id\": o.TypeId}");
    b.push("    for o in doc.Objects if o.TypeId.startswith(\"Part::\")");
    b.push("]");
    let script = b.finish(r#"{"success": True, "objects": objects}"#);
    let val = bridge::run(script, config)?;
    serde_json::from_value(val).map_err(FreeCADError::JsonError)
}

/// Return volume, surface area, and bounding box for a given object.
pub fn get_object_info(input: &ObjectIdInput, config: &Config) -> Result<InfoResult, FreeCADError> {
    if !bridge::object_exists(&input.object_id, config)? {
        return Err(FreeCADError::ObjectNotFound(input.object_id.clone()));
    }
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push(format!("obj = doc.getObject({id:?})", id = input.object_id));
    b.push("shape = obj.Shape");
    b.push("bb = shape.BoundBox");
    b.push("result = {");
    b.push("    \"success\": True,");
    b.push("    \"id\": obj.Name,");
    b.push("    \"label\": obj.Label,");
    b.push("    \"type_id\": obj.TypeId,");
    b.push("    \"volume_mm3\": shape.Volume,");
    b.push("    \"surface_area_mm2\": shape.Area,");
    b.push("    \"bounding_box\": {");
    b.push("        \"xmin\": bb.XMin, \"xmax\": bb.XMax,");
    b.push("        \"ymin\": bb.YMin, \"ymax\": bb.YMax,");
    b.push("        \"zmin\": bb.ZMin, \"zmax\": bb.ZMax,");
    b.push("    },");
    b.push("}");
    let script = b.finish("result");
    let val = bridge::run(script, config)?;
    serde_json::from_value(val).map_err(FreeCADError::JsonError)
}

/// Return overall stats for the session document.
pub fn get_document_stats(config: &Config) -> Result<StatsResult, FreeCADError> {
    let mut b = ScriptBuilder::new(&config.session_doc);
    b.push("part_objs = [o for o in doc.Objects if o.TypeId.startswith(\"Part::\") and hasattr(o, 'Shape')]");
    b.push("total_vol = sum(o.Shape.Volume for o in part_objs)");
    b.push("result = {");
    b.push("    \"success\": True,");
    b.push("    \"object_count\": len(part_objs),");
    b.push("    \"total_volume_mm3\": total_vol,");
    b.push("    \"units\": \"mm\",");
    b.push("    \"session_doc\": DOC_PATH,");
    b.push("}");
    let script = b.finish("result");
    let val = bridge::run(script, config)?;
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
    fn list_objects_returns_err_when_binary_missing() {
        assert!(list_objects(&fixture_config()).is_err());
    }

    #[test]
    fn get_object_info_returns_not_found_or_err_when_binary_missing() {
        let input = ObjectIdInput { object_id: "Box001".into() };
        assert!(get_object_info(&input, &fixture_config()).is_err());
    }

    #[test]
    fn get_document_stats_returns_err_when_binary_missing() {
        assert!(get_document_stats(&fixture_config()).is_err());
    }
}
