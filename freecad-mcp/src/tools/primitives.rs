use crate::config::Config;
use crate::error::FreeCADError;

/// Create a box primitive. Returns object_id and label.
pub fn create_box(_input: &super::CreateBoxInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}

/// Create a cylinder primitive. Returns object_id and label.
pub fn create_cylinder(_input: &super::CreateCylinderInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}

/// Create a sphere primitive. Returns object_id and label.
pub fn create_sphere(_input: &super::CreateSphereInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}

/// Create a cone primitive. Returns object_id and label.
pub fn create_cone(_input: &super::CreateConeInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}
