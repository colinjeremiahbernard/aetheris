use aetheris_core::replay::analyze_point;
use aetheris_core::{Severity, TelemetryPoint};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    satellite_id: String,
}

#[derive(Debug, Deserialize)]
struct DetectionRequest {
    time: DateTime<Utc>,
    subsystem: String,
    sensor_id: String,
    value: f64,
    unit: String,
    quality_flag: i16,
}

#[derive(Debug, Serialize)]
struct DetectionApiResponse {
    point: TelemetryPoint,
    severity: Option<Severity>,
    anomaly_score: Option<f64>,
    explanation: String,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        satellite_id: "AETHERIS-01".to_string(),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/detect", post(detect_telemetry))
        .with_state(Arc::new(state));

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn detect_telemetry(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DetectionRequest>,
) -> Json<DetectionApiResponse> {
    let point = TelemetryPoint {
        time: req.time,
        satellite_id: state.satellite_id.clone(),
        subsystem: req.subsystem,
        sensor_id: req.sensor_id,
        value: req.value,
        unit: req.unit,
        quality_flag: req.quality_flag as i16,
    };

    let result = analyze_point(point);

    let explanation = match result.severity {
        Some(Severity::Critical) => "Battery temperature is critically high.".to_string(),
        Some(Severity::High) => "Battery temperature is above the warning threshold.".to_string(),
        Some(Severity::Medium) => "Battery temperature is elevated.".to_string(),
        Some(Severity::Low) => "Battery temperature is slightly elevated.".to_string(),
        None => "Telemetry is within normal range.".to_string(),
    };

    Json(DetectionApiResponse {
        point: result.point,
        severity: result.severity,
        anomaly_score: result.anomaly_score,
        explanation,
    })
}
