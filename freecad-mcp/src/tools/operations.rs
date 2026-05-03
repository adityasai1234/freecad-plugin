use crate::config::Config;
use crate::error::FreeCADError;

/// Fuse two objects (Part::Fuse). Returns the resulting object_id.
pub fn boolean_union(_input: &super::BooleanInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}

/// Subtract tool from base (Part::Cut). Returns the resulting object_id.
pub fn boolean_cut(_input: &super::BooleanInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}

/// Intersect two objects (Part::Common). Returns the resulting object_id.
pub fn boolean_intersection(_input: &super::BooleanInput, _config: &Config) -> Result<super::ObjectResult, FreeCADError> {
    todo!()
}
