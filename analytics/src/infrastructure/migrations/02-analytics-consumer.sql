CREATE TABLE IF NOT EXISTS furi.analytics_consumer (
    `date` DateTime('America/Sao_Paulo'),
    `uri` String,
    `ip` String,
    `user_agent` String,
    `country` String,
) ENGINE = Kafka()
SETTINGS 
  kafka_broker_list = 'redpanda:9092', 
  kafka_topic_list = 'analytics',
  kafka_group_name = 'clickhouse',
  kafka_format = 'JSONEachRow',
  kafka_thread_per_consumer = 0, 
  kafka_num_consumers = 1, 
  kafka_flush_interval_ms = 5000;
