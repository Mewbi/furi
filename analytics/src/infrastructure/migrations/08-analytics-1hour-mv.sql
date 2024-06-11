CREATE MATERIALIZED VIEW IF NOT EXISTS furi.analytics_1hour_mv
TO furi.analytics_1hour
AS
SELECT
  toStartOfHour(date) AS date,
  uri,
  device_type,
  country,
  sum(count) AS count
FROM furi.analytics_1min
GROUP BY date, uri, device_type, country
ORDER BY date, uri;
