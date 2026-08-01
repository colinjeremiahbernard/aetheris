use aetheris_core::{Database, TelemetryPoint};
use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
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
        .with_state(db);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Aetheris API listening on port 3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Aetheris is healthy"
}

async fn db_health_check(db: State<Database>) -> Result<&'static str, axum::http::StatusCode> {
    match db.health_check().await {
        Ok(_) => Ok("Database is healthy"),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct TelemetryInput {
    timestamp: String,
    satellite_id: String,
    subsystem: String,
    sensor_id: String,
    value: f64,
    unit: String,
    quality_flag: i16,
}

async fn ingest_telemetry(
    State(db): State<Database>,
    Json(payload): Json<TelemetryInput>,
) -> Result<Json<&'static str>, axum::http::StatusCode> {
    let time = chrono::DateTime::parse_from_rfc3339(&payload.timestamp)
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?
        .with_timezone(&chrono::Utc);
    
    let point = TelemetryPoint {
        time,
        satellite_id: payload.satellite_id,
        subsystem: payload.subsystem,
        sensor_id: payload.sensor_id,
        value: payload.value,
        unit: payload.unit,
        quality_flag: payload.quality_flag as i16,
    };
    
    point.insert(&db.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json("Telemetry ingested successfully"))
}

#[derive(Deserialize)]
struct TelemetryQueryParams {
    satellite_id: Option<String>,
    sensor_id: Option<String>,
    limit: Option<i32>,
}

async fn query_telemetry(
    State(db): State<Database>,
    Query(params): Query<TelemetryQueryParams>,
) -> Result<Json<Vec<TelemetryPoint>>, axum::http::StatusCode> {
    let satellite_id = params.satellite_id.unwrap_or_else(|| "OPS-SAT".to_string());
    let limit = params.limit.unwrap_or(100);
    
    let points = match params.sensor_id {
        Some(sensor_id) => {
            sqlx::query_as::<_, TelemetryPoint>(
                r#"
                SELECT time, satellite_id, subsystem, sensor_id, value, unit, quality_flag
                FROM telemetry
                WHERE satellite_id = $1 AND sensor_id = $2
                ORDER BY time DESC
                LIMIT $3
                "#,
            )
            .bind(&satellite_id)
            .bind(&sensor_id)
            .bind(limit)
            .fetch_all(&db.pool)
            .await
        }
        None => {
            sqlx::query_as::<_, TelemetryPoint>(
                r#"
                SELECT time, satellite_id, subsystem, sensor_id, value, unit, quality_flag
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
    .map_err(|e| {
        tracing::error!("Query error: {:?}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(points))
}