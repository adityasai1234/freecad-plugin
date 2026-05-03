use std::sync::Arc;

use crate::config::Config;

pub struct FreeCADMcpServer {
    pub config: Arc<Config>,
}

impl FreeCADMcpServer {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
