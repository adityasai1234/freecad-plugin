use crate::config::Config;
use crate::error::FreeCADError;

/// Return the minimum distance in mm between two objects.
pub fn measure_distance(_input: &super::MeasureInput, _config: &Config) -> Result<super::DistanceResult, FreeCADError> {
    todo!()
}
