CREATE TABLE IF NOT EXISTS furi.analytics_1min (
  `date` DateTime('America/Sao_Paulo'),
  `uri` String,
  `device_type` LowCardinality(String),
  `country` LowCardinality(String),
  `count` UInt64
) ENGINE = SummingMergeTree()
ORDER BY (date, uri, device_type, country);
