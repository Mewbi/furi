-- 1-minute aggregation (real-time enabled)
CREATE MATERIALIZED VIEW IF NOT EXISTS analytics_1min
WITH (timescaledb.continuous, timescaledb.materialized_only = false) AS
SELECT
    time_bucket('1 minute', time) AS bucket,
    uri,
    extract_device_type(user_agent) AS device_type,
    country,
    COUNT(*) AS count
FROM analytics_raw
GROUP BY bucket, uri, extract_device_type(user_agent), country
WITH NO DATA;

-- 1-hour aggregation
CREATE MATERIALIZED VIEW IF NOT EXISTS analytics_1hour
WITH (timescaledb.continuous, timescaledb.materialized_only = false) AS
SELECT
    time_bucket('1 hour', time) AS bucket,
    uri,
    extract_device_type(user_agent) AS device_type,
    country,
    COUNT(*) AS count
FROM analytics_raw
GROUP BY bucket, uri, extract_device_type(user_agent), country
WITH NO DATA;

-- 1-day aggregation
CREATE MATERIALIZED VIEW IF NOT EXISTS analytics_1day
WITH (timescaledb.continuous, timescaledb.materialized_only = false) AS
SELECT
    time_bucket('1 day', time) AS bucket,
    uri,
    extract_device_type(user_agent) AS device_type,
    country,
    COUNT(*) AS count
FROM analytics_raw
GROUP BY bucket, uri, extract_device_type(user_agent), country
WITH NO DATA;
