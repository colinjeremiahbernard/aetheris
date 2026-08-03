use crate::{detect, Severity, TelemetryPoint};

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub point: TelemetryPoint,
    pub severity: Option<Severity>,
    pub anomaly_score: Option<f64>,
}

pub fn analyze_point(point: TelemetryPoint) -> DetectionResult {
    let anomaly = detect(&point);

    DetectionResult {
        point,
        severity: anomaly.as_ref().map(|item| item.severity.clone()),
        anomaly_score: anomaly.map(|item| item.anomaly_score),
    }
}