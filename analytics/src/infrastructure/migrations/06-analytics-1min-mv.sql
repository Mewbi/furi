CREATE MATERIALIZED VIEW IF NOT EXISTS furi.analytics_1min_mv
TO 
  furi.analytics_1min
AS
SELECT
  toStartOfMinute(date) as date,
  uri,
  CASE
    WHEN lower(user_agent) LIKE '%linux%android%' THEN toLowCardinality('Android')
    WHEN lower(user_agent) LIKE '%iphone%'        THEN toLowCardinality('Iphone')
    WHEN lower(user_agent) LIKE '%linux%'         THEN toLowCardinality('Linux')
    WHEN lower(user_agent) LIKE '%windows%'       THEN toLowCardinality('Windows')
    WHEN lower(user_agent) LIKE '%mac os%'        THEN toLowCardinality('Mac')
    ELSE 'Other'
  END as device_type,
  toLowCardinality(country) as country,
  count() as count
FROM 
  furi.analytics_raw
GROUP BY date, uri, device_type, country
ORDER BY date, uri
