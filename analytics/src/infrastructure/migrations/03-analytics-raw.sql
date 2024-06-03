CREATE TABLE IF NOT EXISTS furi.analytics_raw (
  `date` DateTime('America/Sao_Paulo'),
  `uri` String,
  `ip` String,
  `user_agent` String,
  `country` LowCardinality(String),
) ENGINE = MergeTree()
ORDER BY date;
