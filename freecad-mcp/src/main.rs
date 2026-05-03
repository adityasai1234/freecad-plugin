use std::sync::Arc;

use anyhow::Context;
use freecad_mcp::bridge::ensure_workspace;
use freecad_mcp::config::Config;
use freecad_mcp::mcp_server::FreeCADMcpServer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Arc::new(Config::from_env().context("loading configuration")?);
    config
        .validate()
        .context("FREECADCMD_PATH must exist and work dir must be writable")?;
    ensure_workspace(&config).context("creating workspace directory")?;

    let server = FreeCADMcpServer::new(config);
    let io = rmcp::transport::stdio();
    rmcp::serve_server(server, io)
        .await
        .map_err(anyhow::Error::from)?
        .waiting()
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
