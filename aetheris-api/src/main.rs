mod detection;
mod handlers;
mod models;

use aetheris_core::{detect, Database};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use detection::TelemetryInput;
use handlers::anomalies::query_anomalies;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/aetheris".to_string());

    let db = Database::new(&database_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/db-health", get(db_health_check))
        .route("/api/v1/telemetry/detect", post(detect_telemetry))
        .route("/api/v1/anomalies", get(query_anomalies))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    tracing::info!("Aetheris API listening on port 3000");

    axum::serve(listener, app).await.expect("API server failed");
}

async fn health_check() -> &'static str {
    "Aetheris is healthy"
}

async fn db_health_check(State(db): State<Database>) -> Result<&'static str, StatusCode> {
    db.health_check()
        .await
        .map(|_| "Database is healthy")
        .map_err(|error| {
            tracing::error!("Database health check failed: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn detect_telemetry(
    State(db): State<Database>,
    Json(payload): Json<TelemetryInput>,
) -> Result<Json<aetheris_core::Anomaly>, StatusCode> {
    let point = payload.into_point()?;

    let anomaly = detect(&point).ok_or(StatusCode::NO_CONTENT)?;

    sqlx::query(
        r#"
        INSERT INTO anomalies
            (
                id,
                time,
                satellite_id,
                sensor_id,
                anomaly_score,
                anomaly_type,
                severity
            )
        VALUES
            ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(anomaly.id)
    .bind(anomaly.time)
    .bind(&anomaly.satellite_id)
    .bind(&anomaly.sensor_id)
    .bind(anomaly.anomaly_score)
    .bind(format!("{:?}", anomaly.anomaly_type))
    .bind(format!("{:?}", anomaly.severity))
    .execute(&db.pool)
    .await
    .map_err(|error| {
        tracing::error!("Anomaly insert failed: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(anomaly))
}
