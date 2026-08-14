use aetheris_core::replay::analyze_point;
use aetheris_core::{Severity, TelemetryPoint};
use axum::{
    extract::State,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::IntervalStream;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

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

#[derive(Debug, Serialize)]
struct TelemetryLiveEvent {
    time: DateTime<Utc>,
    satellite_id: String,
    subsystem: String,
    sensor_id: String,
    value: f64,
    unit: String,
    quality_flag: i16,
    severity: Option<Severity>,
    anomaly_score: Option<f64>,
    explanation: String,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        satellite_id: "AETHERIS-01".to_string(),
    };
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/detect", post(detect_telemetry))
        .route("/telemetry/stream", get(telemetry_stream))
        .nest_service("/", ServeDir::new("aetheris-web"))
        .layer(cors)
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
        quality_flag: req.quality_flag,
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

async fn telemetry_stream(
    State(state): State<Arc<AppState>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    // Cycle through readings that cover normal, high, and critical severity.
    let readings: &'static [(&str, f64)] = &[
        ("battery_temp_1", 65.0),
        ("battery_temp_1", 68.5),
        ("battery_temp_1", 72.0),
        ("battery_temp_1", 75.0),
        ("battery_temp_1", 81.0),
        ("battery_temp_1", 75.0),
        ("battery_temp_1", 72.0),
        ("battery_temp_1", 68.5),
    ];
    let counter = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let interval = tokio::time::interval(Duration::from_secs(3));
    let stream = IntervalStream::new(interval).map(move |_| {
        let idx = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % readings.len();
        let (sensor_id, value) = readings[idx];
        let point = TelemetryPoint {
            time: Utc::now(),
            satellite_id: state.satellite_id.clone(),
            subsystem: "power".to_string(),
            sensor_id: sensor_id.to_string(),
            value,
            unit: "celsius".to_string(),
            quality_flag: 1,
        };

        let result = analyze_point(point);

        let explanation = match result.severity {
            Some(Severity::Critical) => "Battery temperature is critically high.".to_string(),
            Some(Severity::High) => "Battery temperature is above the warning threshold.".to_string(),
            Some(Severity::Medium) => "Battery temperature is elevated.".to_string(),
            Some(Severity::Low) => "Battery temperature is slightly elevated.".to_string(),
            None => "Telemetry is within normal range.".to_string(),
        };

        let event = TelemetryLiveEvent {
            time: result.point.time,
            satellite_id: result.point.satellite_id,
            subsystem: result.point.subsystem,
            sensor_id: result.point.sensor_id,
            value: result.point.value,
            unit: result.point.unit,
            quality_flag: result.point.quality_flag,
            severity: result.severity,
            anomaly_score: result.anomaly_score,
            explanation,
        };

        Ok(Event::default().event("telemetry").json_data(event).unwrap())
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}