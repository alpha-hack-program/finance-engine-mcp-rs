use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager, StreamableHttpServerConfig,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};
mod common;
use common::{finance_engine::FinanceEngine, metrics};
use axum::{
    response::IntoResponse,
    http::{StatusCode, Request, header},
    middleware::{self, Next},
    body::Body,
};
use std::time::Duration;

const BIND_ADDRESS: &str = "127.0.0.1:8001";
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

/// Middleware to log and fix Accept header for MCP compatibility
async fn fix_accept_header(
    mut request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    // Extract header values as owned Strings before any mutable borrows
    let accept = request
        .headers()
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_default();
    
    let content_type = request
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    // Log incoming headers for debugging
    tracing::info!(
        "Incoming request - Accept: {:?}, Content-Type: {:?}",
        accept,
        content_type
    );
    
    let has_json = accept.contains("application/json");
    let has_sse = accept.contains("text/event-stream");
    
    if !has_json || !has_sse {
        let new_accept = "application/json, text/event-stream";
        request.headers_mut().insert(
            header::ACCEPT,
            new_accept.parse().unwrap(),
        );
        tracing::warn!(
            "Fixed Accept header: '{}' -> '{}'", 
            accept, 
            new_accept
        );
    }
    
    next.run(request).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| BIND_ADDRESS.to_string());
    tracing::info!("Starting streamable-http Finance Engine MCP server on {}", bind_address);
    
    let service = StreamableHttpService::new(
        || Ok(FinanceEngine::new()),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig {
            sse_retry: None,
            ..Default::default()
        },
    );

    // Create router with the MCP service at root
    let mcp_router = axum::Router::new()
        .fallback_service(service)
        .layer(middleware::from_fn(fix_accept_header));

    let router = axum::Router::new()
        .nest("/mcp", mcp_router)
        .route("/metrics", axum::routing::get(metrics_handler))
        .route("/health", axum::routing::get(health_handler));
    
    let tcp_listener = tokio::net::TcpListener::bind(bind_address).await?;
    
    tracing::info!("Server started. Press Ctrl+C to stop.");
    
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
            tracing::info!("Shutdown signal received, stopping server...");
            
            // Force exit after timeout if graceful shutdown hangs
            tokio::spawn(async {
                tokio::time::sleep(SHUTDOWN_TIMEOUT).await;
                tracing::warn!("Force exit after {:?} timeout", SHUTDOWN_TIMEOUT);
                std::process::exit(0);
            });
        })
        .await?;
    
    tracing::info!("Server stopped");
    Ok(())
}

async fn metrics_handler() -> impl IntoResponse {
    let output = metrics::METRICS.gather();
    (StatusCode::OK, output)
}

async fn health_handler() -> impl IntoResponse {
    let output = "OK";
    (StatusCode::OK, output)
}