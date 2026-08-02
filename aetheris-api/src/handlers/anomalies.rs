use crate::models::{AnomalyQueryParams, AnomalyRecord};
use aetheris_core::Database;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

pub async fn query_anomalies(
    State(db): State<Database>,
    Query(params): Query<AnomalyQueryParams>,
) -> Result<Json<Vec<AnomalyRecord>>, StatusCode> {
    let satellite_id = params
        .satellite_id
        .unwrap_or_else(|| "OPS-SAT".to_string());

    let severity = params.severity;
    let limit = params.limit.unwrap_or(100).clamp(1, 1_000);

    let records = match severity {
        Some(severity) => {
            sqlx::query_as::<_, AnomalyRecord>(
                r#"
                SELECT
                    id,
                    time,
                    satellite_id,
                    sensor_id,
                    anomaly_score,
                    anomaly_type,
                    severity,
                    created_at
                FROM anomalies
                WHERE satellite_id = $1
                  AND severity = $2
                ORDER BY time DESC
                LIMIT $3
                "#,
            )
            .bind(&satellite_id)
            .bind(severity)
            .bind(limit)
            .fetch_all(&db.pool)
            .await
        }
        None => {
            sqlx::query_as::<_, AnomalyRecord>(
                r#"
                SELECT
                    id,
                    time,
                    satellite_id,
                    sensor_id,
                    anomaly_score,
                    anomaly_type,
                    severity,
                    created_at
                FROM anomalies
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
        tracing::error!("Anomaly query failed: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(records))
}