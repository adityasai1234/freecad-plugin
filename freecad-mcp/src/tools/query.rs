use crate::config::Config;
use crate::error::FreeCADError;

/// List all Part objects in the session document.
pub fn list_objects(_config: &Config) -> Result<super::ListResult, FreeCADError> {
    todo!()
}

/// Return volume, surface area, and bounding box for a given object.
pub fn get_object_info(_input: &super::ObjectIdInput, _config: &Config) -> Result<super::InfoResult, FreeCADError> {
    todo!()
}

/// Return overall stats for the session document (object count, total volume, units).
pub fn get_document_stats(_config: &Config) -> Result<super::StatsResult, FreeCADError> {
    todo!()
}
