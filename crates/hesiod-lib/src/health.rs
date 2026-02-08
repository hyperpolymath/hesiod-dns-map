// SPDX-License-Identifier: PMPL-1.0-or-later
//! HTTP health and metrics endpoints using Axum (port 8080).

use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::{get, post};
use axum::Router;
use serde_json::{json, Value};
use tracing::info;

use crate::server::DnsServerState;

/// Build the Axum router for health/metrics endpoints.
pub fn health_router(state: Arc<DnsServerState>) -> Router {
    Router::new()
        .route("/dns/health", get(health_check))
        .route("/dns/metrics", get(metrics))
        .route("/dns/reload", post(reload))
        .with_state(state)
}

/// `GET /dns/health` - Returns server status, zone record count, and uptime.
async fn health_check(State(state): State<Arc<DnsServerState>>) -> Json<Value> {
    let uptime = state.start_time.elapsed();
    Json(json!({
        "status": "healthy",
        "zone_records": state.zone.record_count(),
        "domain": state.zone.domain,
        "uptime_seconds": uptime.as_secs(),
    }))
}

/// `GET /dns/metrics` - Returns query count and performance metrics.
async fn metrics(State(state): State<Arc<DnsServerState>>) -> Json<Value> {
    let query_count = state
        .query_count
        .load(std::sync::atomic::Ordering::Relaxed);
    let uptime = state.start_time.elapsed().as_secs();
    let qps = if uptime > 0 {
        query_count as f64 / uptime as f64
    } else {
        0.0
    };
    Json(json!({
        "query_count": query_count,
        "uptime_seconds": uptime,
        "queries_per_second": qps,
        "zone_records": state.zone.record_count(),
    }))
}

/// `POST /dns/reload` - Placeholder for zone reload (returns acknowledgement).
async fn reload(State(_state): State<Arc<DnsServerState>>) -> (StatusCode, Json<Value>) {
    // In a full implementation this would re-read the config and rebuild the zone.
    // For now it acknowledges the request.
    info!("zone reload requested");
    (
        StatusCode::OK,
        Json(json!({
            "status": "acknowledged",
            "message": "zone reload is not yet implemented in this version",
        })),
    )
}

/// Start the HTTP health server on the given port.
pub async fn run_health_server(state: Arc<DnsServerState>, port: u16) -> anyhow::Result<()> {
    let app = health_router(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Health/metrics HTTP server listening on port {}", port);
    axum::serve(listener, app).await?;
    Ok(())
}
