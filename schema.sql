CREATE DATABASE IF NOT EXISTS furi;

CREATE TABLE IF NOT EXISTS furi.analytics_consumer (
    `key` String,
    `value` String
) ENGINE = Kafka()
SETTINGS 
  kafka_broker_list = 'redpanda:9092', 
  kafka_topic_list = 'analytics',
  kafka_group_name = 'clickhouse',
  kafka_format = 'JSONEachRow',
  kafka_thread_per_consumer = 0, 
  kafka_num_consumers = 1, 
  kafka_flush_interval_ms = 5000;

CREATE TABLE IF NOT EXISTS furi.analytics_raw (
    `key` String,
    `value` String
) ENGINE = MergeTree()
ORDER BY key;


CREATE MATERIALIZED VIEW IF NOT EXISTS furi.analytics_mv TO furi.analytics_raw
AS SELECT 
  * 
FROM furi.analytics_consumer;
