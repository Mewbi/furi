# Furi

A Fast way to generate a short URI, in others words it's an URL shortener

## Clickhouse

To access the database execute

```sh
docker exec -it clickhouse clickhouse-client
```

## Redpanda

To send some data manually to a topic use `kafkacat`

```sh
echo '{"key":"key1", "value":"value1"}' | kafkacat -b localhost:9092 -t analytics -P
```

To consume from a topic

```sh
kafkacat -b localhost:9092 -t analytics -C -o beginning
```
