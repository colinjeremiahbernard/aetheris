# Aetheris

**AI-Powered Spacecraft Telemetry Intelligence Platform**

A high-performance Rust-based system for real-time spacecraft telemetry monitoring, anomaly detection, and predictive maintenance using machine learning. So it is definitely an AI-powered spacecraft health intelligence system that continuously analyzes satellite telemetry, detects emerging anomalies, explain their likely causes, predicts equipment degradation, and recommends mission-operations actions.

## 🎯 Project Goal

Transform raw spacecraft telemetry into actionable insights using AI-driven anomaly detection, predictive monitoring, and natural language diagnostics—making space operations more accessible and data-driven.

## 🏗️ Architecture

┌─────────────────────────────────────────────────────────────────┐
│ Aetheris Architecture │
├─────────────────────────────────────────────────────────────────┤
│ │
│ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│ │ Data Ingest │ → │ Processing │ → │ ML Engine │ │
│ │ (Rust) │ │ (Rust) │ │ (Rust/ONNX) │ │
│ └──────────────┘ └──────────────┘ └──────────────┘ │
│ ↓ ↓ ↓ │
│ ┌──────────────────────────────────────────────────────────┐ │
│ │ PostgreSQL + TimescaleDB (Telemetry Store) │ │
│ └──────────────────────────────────────────────────────────┘ │
│ ↓ ↓ ↓ │
│ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│ │ REST API │ │ WebSocket │ │ Query API │ │
│ │ (axum) │ │ (Real-time) │ │ (NLP-powered)│ │
│ └──────────────┘ └──────────────┘ └──────────────┘ │
│ │
│ ┌──────────────────────────────────────────────────────────┐ │
│ │ Frontend (Streamlit/Python Dashboard) │ │
│ └──────────────────────────────────────────────────────────┘ │
│ │
└─────────────────────────────────────────────────────────────────┘

## 🛠️ Technology Stack

| Component         | Technology               | Purpose                                            |
| ----------------- | ------------------------ | -------------------------------------------------- |
| **Core Engine**   | Rust (tokio, async)      | High-performance, memory-safe telemetry processing |
| **ML Inference**  | Rust + ONNX Runtime      | Production-grade anomaly detection models          |
| **Database**      | PostgreSQL + TimescaleDB | Time-series optimized telemetry storage            |
| **API Layer**     | Rust (axum)              | High-throughput REST + WebSocket APIs              |
| **Message Queue** | Redis Streams            | Real-time telemetry streaming                      |
| **Frontend**      | Python (Streamlit)       | Interactive dashboards and visualization           |
| **Deployment**    | Docker + Kubernetes      | Scalable cloud and edge deployment                 |

## 📦 Project Structure

aetheris/
├── aetheris-core/ # Core Rust library
│ ├── src/
│ │ ├── lib.rs
│ │ ├── telemetry.rs # Telemetry data models
│ │ └── database.rs # Database connection + migrations
│ ├── migrations/
│ │ └── 1_initial_schema.sql
│ └── Cargo.toml
│
├── aetheris-api/ # REST + WebSocket API server
│ ├── src/
│ │ └── main.rs
│ └── Cargo.toml
│
├── aetheris-ml/ # ML training (Python) [TODO]
│ ├── train_models.py
│ └── requirements.txt
│
├── aetheris-frontend/ # Streamlit dashboard [TODO]
│ └── app.py
│
├── docker/ # Docker configurations [TODO]
│ └── docker-compose.yml
│
├── Cargo.toml # Workspace configuration
└── README.md

## 🚀 Features (Planned)

### Core Capabilities

- ✅ **Multi-Model Anomaly Detection** - Ensemble of ML/DL models (Random Forest, LSTM, Transformers)
- 🔄 **Real-Time Telemetry Processing** - Sub-millisecond latency for time-critical operations
- 🔄 **Natural Language Queries** - Ask questions like "Show me temperature anomalies in the last 24h"
- 🔄 **Automated Diagnostic Reports** - AI-generated explanations with SHAP attribution
- 🔄 **Predictive Maintenance** - Forecast component failures before they occur
- 🔄 **Multi-Satellite Constellation View** - Track health across entire satellite fleets

### AI Components

- **Anomaly Detection Engine**: 90%+ accuracy on ESA OPS-SAT benchmark dataset
- **Edge-Optimized Models**: <100KB model size for on-board deployment
- **LLM-Powered Diagnostics**: Natural language explanations for detected anomalies
- **Time-Series Forecasting**: Predictive models for subsystem health trends

## 🏃 Quick Start

### Prerequisites

- Rust 1.70+ (with Cargo)
- PostgreSQL 15+ (or TimescaleDB)
- Docker (optional, for containerized deployment)

### Database Setup

```bash
# Set database connection URL
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/aetheris"

# Or on PowerShell
$env:DATABASE_URL="postgres://postgres:postgres@localhost:5432/aetheris"
```

Database migrations run automatically on first start.

### Run the API

```bash
# Build
cargo build

# Run API server
cargo run -p aetheris-api
```

### Test Endpoints

```bash
# Health check
curl http://localhost:3000/health

# Database health check
curl http://localhost:3000/api/v1/db-health
```

## 📊 Database Schema

### Telemetry (Hypertable)

- `time` - Timestamp (TIMESTAMPTZ)
- `satellite_id` - Satellite identifier
- `subsystem` - Subsystem name (e.g., "power", "thermal")
- `sensor_id` - Sensor identifier
- `value` - Sensor reading (DOUBLE PRECISION)
- `unit` - Measurement unit
- `quality_flag` - Data quality indicator

### Anomalies

- `id` - Unique identifier (UUID)
- `time` - Detection timestamp
- `satellite_id` - Affected satellite
- `sensor_id` - Affected sensor
- `anomaly_score` - Confidence score (0-1)
- `anomaly_type` - Classification (point, contextual, collective, drift, sensor_failure)
- `severity` - Low, Medium, High, Critical
- `created_at` - Record creation time

### Satellites

- `id` - Unique identifier
- `name` - Satellite name
- `operator` - Operating organization
- `orbit_type` - LEO, MEO, GEO, etc.
- `launch_date` - Launch date
- `status` - active, inactive, decommissioned

## 🔧 Development

### Running Tests

```bash
cargo test
```

### Database Migrations

Migrations are located in `aetheris-core/migrations/` and run automatically.

To add a new migration:

```bash
# Create new migration file
touch aetheris-core/migrations/2_add_new_feature.sql
```

### Code Style

```bash
cargo fmt
cargo clippy
```

## 📈 Performance Targets

- **Telemetry Ingestion**: 10,000+ points/second
- **Anomaly Detection Latency**: <10ms per sensor reading
- **API Response Time**: <100ms p95
- **Model Size (Edge)**: <100KB
- **Detection Accuracy**: >90% on OPS-SAT benchmark

## 🎓 Datasets

- **ESA OPS-SAT**: Real spacecraft telemetry with labeled anomalies
- **NASA SMAP**: Soil Moisture Active Passive satellite data
- **Kaggle Spacecraft Telemetry**: Public anomaly detection datasets

## 📝 License

MIT License - See LICENSE file for details

## 🤝 Contributing

Contributions welcome! Please open an issue or submit a PR.

---

**Built with ❤️ for space exploration**

# Aetheris - Development Guide for AI Assistants

## Project Overview

Aetheris is an AI-powered spacecraft telemetry intelligence platform built in Rust for high-performance, real-time anomaly detection and predictive maintenance.

## Tech Stack

- **Backend**: Rust (tokio, axum, sqlx)
- **Database**: PostgreSQL + TimescaleDB
- **ML**: Python (training) → ONNX (Rust inference)
- **Frontend**: Python Streamlit

## Key Directories

- `aetheris-core/` - Core library with telemetry models and database layer
- `aetheris-api/` - REST API server using axum
- `aetheris-core/migrations/` - SQL database migrations

## Development Guidelines

1. **Performance First**: All core processing must be async and optimized for low latency
2. **Type Safety**: Use Rust's type system to prevent telemetry data errors
3. **Error Handling**: Use `thiserror` and `anyhow` appropriately
4. **Database**: All queries use sqlx with compile-time verification
5. **Migrations**: Never modify existing migrations; always create new ones

## Testing Strategy

- Unit tests for core logic in `aetheris-core`
- Integration tests for API endpoints
- Load tests for telemetry ingestion pipeline

## Deployment

- Docker containers for each service
- Kubernetes for orchestration
- Edge deployment optimized for <500MB memory footprint
