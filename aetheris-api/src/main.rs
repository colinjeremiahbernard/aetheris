mod handlers;
mod models;

use aetheris_core::{detect, Database, TelemetryPoint};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono;
use handlers::anomalies::query_anomalies;
use serde::{Deserialize, Serialize};
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
        .route("/api/v1/telemetry", post(ingest_telemetry))
        .route("/api/v1/telemetry", get(query_telemetry))
        .route("/api/v1/telemetry/batch", post(ingest_telemetry_batch))
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

#[derive(Debug, Deserialize)]
struct TelemetryInput {
    timestamp: String,
    satellite_id: String,
    subsystem: String,
    sensor_id: String,
    value: f64,
    unit: String,
    quality_flag: i16,
}

impl TelemetryInput {
    fn into_point(self) -> Result<TelemetryPoint, StatusCode> {
        let time = chrono::DateTime::parse_from_rfc3339(&self.timestamp)
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .with_timezone(&chrono::Utc);

        Ok(TelemetryPoint {
            time,
            satellite_id: self.satellite_id,
            subsystem: self.subsystem,
            sensor_id: self.sensor_id,
            value: self.value,
            unit: self.unit,
            quality_flag: self.quality_flag,
        })
    }
}

#[derive(Debug, Serialize)]
struct IngestResponse {
    accepted: usize,
}

async fn ingest_telemetry(
    State(db): State<Database>,
    Json(payload): Json<TelemetryInput>,
) -> Result<Json<IngestResponse>, StatusCode> {
    let point = payload.into_point()?;

    point.insert(&db.pool).await.map_err(|error| {
        tracing::error!("Telemetry insert failed: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(IngestResponse { accepted: 1 }))
}

#[derive(Debug, Deserialize)]
struct BatchTelemetryInput {
    points: Vec<TelemetryInput>,
}

async fn ingest_telemetry_batch(
    State(db): State<Database>,
    Json(payload): Json<BatchTelemetryInput>,
) -> Result<Json<IngestResponse>, StatusCode> {
    if payload.points.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.points.len() > 1_000 {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }

    let accepted = payload.points.len();

    let mut transaction = db.pool.begin().await.map_err(|error| {
        tracing::error!("Transaction start failed: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    for input in payload.points {
        let point = input.into_point()?;

        sqlx::query(
            r#"
            INSERT INTO telemetry
                (time, satellite_id, subsystem, sensor_id, value, unit, quality_flag)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(point.time)
        .bind(point.satellite_id)
        .bind(point.subsystem)
        .bind(point.sensor_id)
        .bind(point.value)
        .bind(point.unit)
        .bind(point.quality_flag)
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            tracing::error!("Batch telemetry insert failed: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    transaction.commit().await.map_err(|error| {
        tracing::error!("Batch transaction commit failed: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(IngestResponse { accepted }))
}
#[derive(Debug, Deserialize)]
struct TelemetryQueryParams {
    satellite_id: Option<String>,
    sensor_id: Option<String>,
    limit: Option<i32>,
}

async fn query_telemetry(
    State(db): State<Database>,
    Query(params): Query<TelemetryQueryParams>,
) -> Result<Json<Vec<TelemetryPoint>>, StatusCode> {
    let satellite_id = params.satellite_id.unwrap_or_else(|| "OPS-SAT".to_string());

    let limit = params.limit.unwrap_or(100).clamp(1, 1_000);

    let points = match params.sensor_id {
        Some(sensor_id) => {
            sqlx::query_as::<_, TelemetryPoint>(
                r#"
                SELECT
                    time, satellite_id, subsystem, sensor_id,
                    value, unit, quality_flag
                FROM telemetry
                WHERE satellite_id = $1
                  AND sensor_id = $2
                ORDER BY time DESC
                LIMIT $3
                "#,
            )
            .bind(&satellite_id)
            .bind(sensor_id)
            .bind(limit)
            .fetch_all(&db.pool)
            .await
        }
        None => {
            sqlx::query_as::<_, TelemetryPoint>(
                r#"
                SELECT
                    time, satellite_id, subsystem, sensor_id,
                    value, unit, quality_flag
                FROM telemetry
                WHERE satellite_id = $1
                ORDER BY time DESC
                LIMIT $2
                "#,
            )
            .bind(&satellite_id)
            .bind(limit)
            .fetch_all(&db.pool)
            .await
        }
    }
    .map_err(|error| {
        tracing::error!("Telemetry query failed: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(points))
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


