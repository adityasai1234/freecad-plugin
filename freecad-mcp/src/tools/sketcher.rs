use crate::config::Config;
use crate::error::FreeCADError;

/// Create a sketch on the given plane. Returns the sketch object_id.
pub fn create_sketch(_input: &super::CreateSketchInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}

/// Add a rectangle to an existing sketch. Returns edge indices.
pub fn add_rectangle(_input: &super::AddRectangleInput, _config: &Config) -> Result<super::EdgeIndicesResult, FreeCADError> {
    todo!()
}

/// Add a circle to an existing sketch. Returns edge index.
pub fn add_circle(_input: &super::AddCircleInput, _config: &Config) -> Result<super::EdgeIndexResult, FreeCADError> {
    todo!()
}

/// Extrude a sketch into a solid (PartDesign::Pad). Returns the solid object_id.
pub fn extrude(_input: &super::ExtrudeInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}

/// Revolve a sketch around an axis (PartDesign::Revolution). Returns the solid object_id.
pub fn revolve(_input: &super::RevolveInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}
