use anyhow::Result;

use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};

mod common;
use common::finance_engine::FinanceEngine;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Finance Engine MCP server using stdio transport");

    // Create an instance of our finance-engine router
    let service = FinanceEngine::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}