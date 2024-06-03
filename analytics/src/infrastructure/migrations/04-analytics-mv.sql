CREATE MATERIALIZED VIEW IF NOT EXISTS furi.analytics_mv TO furi.analytics_raw
AS SELECT  
  `date`,
  `uri`,
  `ip`,
  `user_agent`,
  toLowCardinality(`country`) AS country
FROM furi.analytics_consumer;
