-- Try to enable TimescaleDB extension (will fail silently if not available)
DO $$ BEGIN CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
EXCEPTION
WHEN OTHERS THEN RAISE NOTICE 'TimescaleDB not available, using standard PostgreSQL';
END $$;
-- Create telemetry table
CREATE TABLE IF NOT EXISTS telemetry (
  time TIMESTAMPTZ NOT NULL,
  satellite_id TEXT NOT NULL,
  subsystem TEXT NOT NULL,
  sensor_id TEXT NOT NULL,
  value DOUBLE PRECISION NOT NULL,
  unit TEXT NOT NULL,
  quality_flag SMALLINT NOT NULL
);
-- Create indexes (works for both TimescaleDB and regular PostgreSQL)
CREATE INDEX IF NOT EXISTS telemetry_time_idx ON telemetry (time DESC);
CREATE INDEX IF NOT EXISTS telemetry_satellite_time_idx ON telemetry (satellite_id, time DESC);
CREATE INDEX IF NOT EXISTS telemetry_sensor_time_idx ON telemetry (sensor_id, time DESC);
CREATE INDEX IF NOT EXISTS telemetry_subsystem_time_idx ON telemetry (subsystem, time DESC);
-- Create anomalies table
CREATE TABLE IF NOT EXISTS anomalies (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  time TIMESTAMPTZ NOT NULL,
  satellite_id TEXT NOT NULL,
  sensor_id TEXT NOT NULL,
  anomaly_score DOUBLE PRECISION NOT NULL,
  anomaly_type TEXT NOT NULL,
  severity TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS anomalies_satellite_time_idx ON anomalies (satellite_id, time DESC);
CREATE INDEX IF NOT EXISTS anomalies_severity_time_idx ON anomalies (severity, time DESC);
-- Create satellites table
CREATE TABLE IF NOT EXISTS satellites (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  operator TEXT,
  orbit_type TEXT,
  launch_date DATE,
  status TEXT NOT NULL DEFAULT 'active'
);
-- Insert sample satellite if not exists
INSERT INTO satellites (id, name, operator, orbit_type, status)
VALUES ('OPS-SAT', 'OPS-SAT', 'ESA', 'LEO', 'active') ON CONFLICT (id) DO NOTHING;