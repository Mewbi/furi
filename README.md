# 🦀 Furi 🦀

<p align="center">
    <img src="./assets/logo_wide.png" height="300">
</p>

A **F**ast way to generate a short **URI**, in others words, it's an URL shortener.

Designed with a high scalability and performance architecture, Furi is a straightforward URL shortener with integrated analytics.

## Architecture

## Tecnologies

## 🏃‍♂️ Running Local

Clone the repository.
```sh
git clone git@github.com:Mewbi/furi.git
```

Set every required configuration and environment file. (Remember to edit the file content)
```sh
cp .env-example .env # Database config
cp ./furi/config-example.toml ./furi/config.prod.toml # Furi API config
cp ./frontend/.env-example ./frontend/.env # Frontend config
```

Generate SSL certificates and place them into the [certs](./certs/) directory. If you are using the default web server configuration, the certificate files must be named `furi-cert.pem` and `furi-key.key`. For more details about this, refer to the [certs](./certs/) directory.

Set the GeoIP database files into the [geoip](./furi/geoip/) directory. The project uses the [MaxMind](https://www.maxmind.com/en/home) GeoIP database, and the default filenames expected are `GeoLite2-City.mmdb` and `GeoLite2-Country.mmdb`. If your files have different names, you can change the Furi config file to match the correct database names. For more details about the GeoIP database, refer to the [geoip](./furi/geoip/) directory.

Then start all services using Docker Compose:
```sh
docker compose up -d
```

The web server will start by default in port `8443`.

A tip is edit your `/etc/hosts` to point every service hostname to your localhost, so you can simulate access using the domain name like:

```
127.0.0.1 redpanda redis postgres clickhouse furi.live www.furi.live
```

### Useful Tools

To access the `clickhouse`, execute

```sh
docker exec -it clickhouse clickhouse-client
```

It is possible to manually send data to a `redpanda` topic using `kafkacat`:

```
echo '{"key":"key1", "value":"value1"}' | kafkacat -b redpanda:9092 -t my-topic -P
``` 

To consume a `redpanda` topic from beginning
```
kafkacat -b redpanda:9092 -t my-topic -C -o beginning
```

## 🤝 Contributing

Furi is an open-source project, so contributions are welcome!

Feel free to share suggestions for new features, improvements, and bug fixes.