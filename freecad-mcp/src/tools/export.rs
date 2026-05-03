use crate::config::Config;
use crate::error::FreeCADError;

/// Save the session document in place.
pub fn save_document(_config: &Config) -> Result<super::SaveResult, FreeCADError> {
    todo!()
}

/// Export an object to STL. Returns file path and size in bytes.
pub fn export_stl(_input: &super::ExportInput, _config: &Config) -> Result<super::ExportResult, FreeCADError> {
    todo!()
}

/// Export an object to STEP. Returns file path and size in bytes.
pub fn export_step(_input: &super::ExportInput, _config: &Config) -> Result<super::ExportResult, FreeCADError> {
    todo!()
}

/// Export an object to OBJ. Returns file path and size in bytes.
pub fn export_obj(_input: &super::ExportInput, _config: &Config) -> Result<super::ExportResult, FreeCADError> {
    todo!()
}
