//! MCP server wiring: exposes FreeCAD tools to Claude via `rmcp` macros.

use std::sync::Arc;

use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{tool, ServerHandler};

use crate::config::Config;
use crate::error::FreeCADError;
use crate::tools::{
    AddCircleInput, AddRectangleInput, BooleanInput, CreateBoxInput, CreateConeInput,
    CreateCylinderInput, CreateSketchInput, CreateSphereInput, ExportInput, ExtrudeInput,
    MeasureInput, ObjectIdInput, PlaceObjectInput, RevolveInput, RotateObjectInput,
};

fn serialize_tool_result<T: serde::Serialize>(res: Result<T, FreeCADError>) -> String {
    match res {
        Ok(v) => serde_json::to_string(&v).unwrap_or_else(|_| r#"{"success":false}"#.into()),
        Err(e) => serde_json::json!({ "success": false, "error": e.to_string() }).to_string(),
    }
}

async fn run_blocking_json<T, F>(cfg: Arc<Config>, f: F) -> String
where
    T: serde::Serialize + Send + 'static,
    F: FnOnce(&Config) -> Result<T, FreeCADError> + Send + 'static,
{
    tokio::task::spawn_blocking(move || serialize_tool_result(f(&cfg)))
        .await
        .unwrap_or_else(|e| {
            serde_json::json!({ "success": false, "error": format!("task failed: {e}") }).to_string()
        })
}

/// MCP service holding shared [`Config`].
#[derive(Clone)]
pub struct FreeCADMcpServer {
    pub config: Arc<Config>,
}

impl FreeCADMcpServer {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[tool(tool_box)]
impl FreeCADMcpServer {
    #[tool(description = "Create a rectangular box primitive (Part::Box). Length, width, and height are in millimetres. Returns object_id for later references.")]
    pub async fn create_box(&self, #[tool(aggr)] input: CreateBoxInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::primitives::create_box(&input, c)).await
    }

    #[tool(description = "Create a cylinder (Part::Cylinder). Radius and height are in millimetres.")]
    pub async fn create_cylinder(&self, #[tool(aggr)] input: CreateCylinderInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::primitives::create_cylinder(&input, c)).await
    }

    #[tool(description = "Create a sphere (Part::Sphere). Radius is in millimetres.")]
    pub async fn create_sphere(&self, #[tool(aggr)] input: CreateSphereInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::primitives::create_sphere(&input, c)).await
    }

    #[tool(description = "Create a cone/truncated cone (Part::Cone). radii and height are in millimetres.")]
    pub async fn create_cone(&self, #[tool(aggr)] input: CreateConeInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::primitives::create_cone(&input, c)).await
    }

    #[tool(description = "Move a Part object to world-space x,y,z in mm. Optional yaw/pitch/roll degrees update rotation.")]
    pub async fn place_object(&self, #[tool(aggr)] input: PlaceObjectInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::placement::place_object(&input, c)).await
    }

    #[tool(description = "Set object orientation from yaw/pitch/roll Euler angles in degrees.")]
    pub async fn rotate_object(&self, #[tool(aggr)] input: RotateObjectInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::placement::rotate_object(&input, c)).await
    }

    #[tool(description = "Boolean fuse two solids (Part::Fuse). Provide base_id and tool_id object names.")]
    pub async fn boolean_union(&self, #[tool(aggr)] input: BooleanInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::operations::boolean_union(&input, c)).await
    }

    #[tool(description = "Boolean cut tool from base (Part::Cut).")]
    pub async fn boolean_cut(&self, #[tool(aggr)] input: BooleanInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::operations::boolean_cut(&input, c)).await
    }

    #[tool(description = "Boolean intersection of two solids (Part::Common).")]
    pub async fn boolean_intersection(&self, #[tool(aggr)] input: BooleanInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::operations::boolean_intersection(&input, c)).await
    }

    #[tool(description = "List all Part objects in the session document.")]
    pub async fn list_objects(&self) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, crate::tools::query::list_objects).await
    }

    #[tool(description = "Volume (mm³), surface area (mm²), and bounding box for a Part object.")]
    pub async fn get_object_info(&self, #[tool(aggr)] input: ObjectIdInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::query::get_object_info(&input, c)).await
    }

    #[tool(description = "Summary counts and total volume for the session document.")]
    pub async fn get_document_stats(&self) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, crate::tools::query::get_document_stats).await
    }

    #[tool(description = "Export a shape to binary STL mesh. Optional filename under FREECAD_WORK_DIR.")]
    pub async fn export_stl(&self, #[tool(aggr)] input: ExportInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::export::export_stl(&input, c)).await
    }

    #[tool(description = "Export a shape to STEP (ISO 10303). Optional filename under FREECAD_WORK_DIR.")]
    pub async fn export_step(&self, #[tool(aggr)] input: ExportInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::export::export_step(&input, c)).await
    }

    #[tool(description = "Export a tessellated mesh as Wavefront OBJ.")]
    pub async fn export_obj(&self, #[tool(aggr)] input: ExportInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::export::export_obj(&input, c)).await
    }

    #[tool(description = "Persist the session FreeCAD document (session.FCStd) to disk.")]
    pub async fn save_document(&self) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, crate::tools::export::save_document).await
    }

    #[tool(description = "Create a sketch on plane XY, XZ, or YZ (Sketcher::SketchObject).")]
    pub async fn create_sketch(&self, #[tool(aggr)] input: CreateSketchInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::sketcher::create_sketch(&input, c)).await
    }

    #[tool(description = "Add an axis-aligned rectangle to a sketch (mm in sketch coordinates). Returns geometry indices.")]
    pub async fn add_rectangle(&self, #[tool(aggr)] input: AddRectangleInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::sketcher::add_rectangle(&input, c)).await
    }

    #[tool(description = "Add a circle to a sketch (center cx,cy, radius in mm).")]
    pub async fn add_circle(&self, #[tool(aggr)] input: AddCircleInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::sketcher::add_circle(&input, c)).await
    }

    #[tool(description = "Extrude the first closed wire of a sketch along its normal into a solid (Part::Feature). depth in mm; symmetric optional.")]
    pub async fn extrude(&self, #[tool(aggr)] input: ExtrudeInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::sketcher::extrude(&input, c)).await
    }

    #[tool(description = "Revolve the first closed sketch wire around local X/Y/Z through the sketch origin. angle_deg in degrees.")]
    pub async fn revolve(&self, #[tool(aggr)] input: RevolveInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::sketcher::revolve(&input, c)).await
    }

    #[tool(description = "Minimum distance in mm between two Part solids.")]
    pub async fn measure_distance(&self, #[tool(aggr)] input: MeasureInput) -> String {
        let cfg = self.config.clone();
        run_blocking_json(cfg, move |c| crate::tools::measure::measure_distance(&input, c)).await
    }
}

#[tool(tool_box)]
impl ServerHandler for FreeCADMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "FreeCAD MCP server: create and edit millimetre-based solids headlessly. Object IDs (e.g. Box001) refer to the shared session.FCStd document.".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
