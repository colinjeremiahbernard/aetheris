use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TelemetryPoint {
    pub time: DateTime<Utc>,
    pub satellite_id: String,
    pub subsystem: String,
    pub sensor_id: String,
    pub value: f64,
    pub unit: String,
    pub quality_flag: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Anomaly {
    pub id: Uuid,
    pub time: DateTime<Utc>,
    pub satellite_id: String,
    pub sensor_id: String,
    pub anomaly_score: f64,
    pub anomaly_type: AnomalyType,
    pub severity: Severity,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    PointAnomaly,
    ContextualAnomaly,
    CollectiveAnomaly,
    Drift,
    SensorFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Deserialize)]
pub struct TelemetryQueryParams {
    pub satellite_id: Option<String>,
    pub sensor_id: Option<String>,
    pub subsystem: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<i32>,
}

impl TelemetryPoint {
    pub async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO telemetry (time, satellite_id, subsystem, sensor_id, value, unit, quality_flag)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(self.time)
        .bind(&self.satellite_id)
        .bind(&self.subsystem)
        .bind(&self.sensor_id)
        .bind(self.value)
        .bind(&self.unit)
        .bind(self.quality_flag)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn query_by_sensor(
        pool: &PgPool,
        satellite_id: &str,
        sensor_id: &str,
        limit: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let points = sqlx::query_as::<_, Self>(
            r#"
            SELECT time, satellite_id, subsystem, sensor_id, value, unit, quality_flag
            FROM telemetry
            WHERE satellite_id = $1 AND sensor_id = $2
            ORDER BY time DESC
            LIMIT $3
            "#,
        )
        .bind(satellite_id)
        .bind(sensor_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(points)
    }

    pub async fn query_by_time_range(
        pool: &PgPool,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let points = sqlx::query_as::<_, Self>(
            r#"
            SELECT time, satellite_id, subsystem, sensor_id, value, unit, quality_flag
            FROM telemetry
            WHERE time >= $1 AND time <= $2
            ORDER BY time DESC
            "#,
        )
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

        Ok(points)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionEvidence {
    pub sensor_id: String,
    pub observed_value: f64,
    pub threshold: f64,
    pub reason: String,
}
