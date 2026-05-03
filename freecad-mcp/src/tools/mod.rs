pub mod export;
pub mod measure;
pub mod operations;
pub mod placement;
pub mod primitives;
pub mod query;
pub mod sketcher;

use crate::error::FreeCADError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ─── Primitive inputs ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateBoxInput {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub label: Option<String>,
}

impl CreateBoxInput {
    pub fn validate(&self) -> Result<(), FreeCADError> {
        if self.length <= 0.0 || self.width <= 0.0 || self.height <= 0.0 {
            return Err(FreeCADError::InvalidInput(
                "length, width, height must be positive".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateCylinderInput {
    pub radius: f64,
    pub height: f64,
    pub label: Option<String>,
}

impl CreateCylinderInput {
    pub fn validate(&self) -> Result<(), FreeCADError> {
        if self.radius <= 0.0 || self.height <= 0.0 {
            return Err(FreeCADError::InvalidInput(
                "radius and height must be positive".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateSphereInput {
    pub radius: f64,
    pub label: Option<String>,
}

impl CreateSphereInput {
    pub fn validate(&self) -> Result<(), FreeCADError> {
        if self.radius <= 0.0 {
            return Err(FreeCADError::InvalidInput("radius must be positive".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateConeInput {
    pub radius1: f64,
    pub radius2: f64,
    pub height: f64,
    pub label: Option<String>,
}

impl CreateConeInput {
    pub fn validate(&self) -> Result<(), FreeCADError> {
        if self.radius1 < 0.0 || self.radius2 < 0.0 || self.height <= 0.0 {
            return Err(FreeCADError::InvalidInput(
                "radii must be non-negative and height must be positive".into(),
            ));
        }
        Ok(())
    }
}

// ─── Placement inputs ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlaceObjectInput {
    pub object_id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: Option<f64>,
    pub pitch: Option<f64>,
    pub roll: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RotateObjectInput {
    pub object_id: String,
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
}

// ─── Boolean / query / export / measure inputs ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BooleanInput {
    pub base_id: String,
    pub tool_id: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ObjectIdInput {
    pub object_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExportInput {
    pub object_id: String,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MeasureInput {
    pub object_id_a: String,
    pub object_id_b: String,
}

// ─── Sketcher inputs ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub enum SketchPlane {
    XY,
    XZ,
    YZ,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateSketchInput {
    pub plane: SketchPlane,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddRectangleInput {
    pub sketch_id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddCircleInput {
    pub sketch_id: String,
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExtrudeInput {
    pub sketch_id: String,
    pub depth: f64,
    pub symmetric: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub enum RevolveAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RevolveInput {
    pub sketch_id: String,
    pub axis: RevolveAxis,
    pub angle_deg: f64,
}

// ─── Output types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectResult {
    pub success: bool,
    pub object_id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectSummary {
    pub id: String,
    pub label: String,
    pub type_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResult {
    pub success: bool,
    pub objects: Vec<ObjectSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoResult {
    pub success: bool,
    pub id: String,
    pub label: String,
    pub type_id: String,
    pub volume_mm3: f64,
    pub surface_area_mm2: f64,
    pub bounding_box: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResult {
    pub success: bool,
    pub object_count: usize,
    pub total_volume_mm3: f64,
    pub units: String,
    pub session_doc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub success: bool,
    pub path: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveResult {
    pub success: bool,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceResult {
    pub success: bool,
    pub distance_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeIndicesResult {
    pub success: bool,
    pub indices: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeIndexResult {
    pub success: bool,
    pub index: i64,
}
