CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS analytics_raw (
    time TIMESTAMPTZ NOT NULL,
    uri TEXT NOT NULL,
    ip TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    country TEXT NOT NULL
);

SELECT create_hypertable('analytics_raw', 'time',
    chunk_time_interval => INTERVAL '1 day',
    if_not_exists => TRUE
);

CREATE INDEX IF NOT EXISTS idx_analytics_raw_uri_time ON analytics_raw (uri, time DESC);
