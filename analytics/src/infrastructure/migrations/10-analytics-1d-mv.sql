CREATE MATERIALIZED VIEW IF NOT EXISTS furi.analytics_1d_mv
TO furi.analytics_1d
AS
SELECT
  toStartOfDay(date) AS date,
  uri,
  device_type,
  country,
  sum(count) AS count
FROM furi.analytics_1hour
GROUP BY date, uri, device_type, country
ORDER BY date, uri;
