pub mod export;
pub mod measure;
pub mod operations;
pub mod placement;
pub mod primitives;
pub mod query;
pub mod sketcher;

// ---------- placeholder types (filled in Commits 15-17) ----------

#[derive(Debug)] pub struct CreateBoxInput;
#[derive(Debug)] pub struct CreateCylinderInput;
#[derive(Debug)] pub struct CreateSphereInput;
#[derive(Debug)] pub struct CreateConeInput;
#[derive(Debug)] pub struct PlaceObjectInput;
#[derive(Debug)] pub struct RotateObjectInput;
#[derive(Debug)] pub struct BooleanInput;
#[derive(Debug)] pub struct ObjectIdInput;
#[derive(Debug)] pub struct ExportInput;
#[derive(Debug)] pub struct CreateSketchInput;
#[derive(Debug)] pub struct AddRectangleInput;
#[derive(Debug)] pub struct AddCircleInput;
#[derive(Debug)] pub struct ExtrudeInput;
#[derive(Debug)] pub struct RevolveInput;
#[derive(Debug)] pub struct MeasureInput;

#[derive(Debug)] pub struct ObjectResult;
#[derive(Debug)] pub struct ListResult;
#[derive(Debug)] pub struct InfoResult;
#[derive(Debug)] pub struct StatsResult;
#[derive(Debug)] pub struct ExportResult;
#[derive(Debug)] pub struct SaveResult;
#[derive(Debug)] pub struct DistanceResult;
#[derive(Debug)] pub struct EdgeIndicesResult;
#[derive(Debug)] pub struct EdgeIndexResult;
