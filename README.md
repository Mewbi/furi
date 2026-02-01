# 🦀 Furi 🦀

<p align="center">
    <img src="./assets/logo_wide.png" height="300">
</p>

A **F**ast way to generate a short **URI**, in others words, it's an URL shortener.

Designed with a high scalability and performance architecture, Furi is a straightforward URL shortener with integrated analytics.

## 🏗️ Architecture

<p align="center">
    <img src="./assets/diagram.png">
</p>

The first layer of access is a CDN platform, in this case, [Cloudflare](https://www.cloudflare.com/). The major reason for using a CDN is security (e.g., WAF), but the frontend static content can also be cached, improving the performance of the website.

After that, the access is sent to a [Web Server](./web-server/). Its responsibilities include serving the frontend static files, acting as a load balancer for the N instances of the Shortener API (in this case, 2 instances), and proxying access to the Analytics API.

The [Shortener API](./furi/) creates and stores the short and original URLs into database (a SQL-like database) and uses a cache layer (key-value database) to improve response performance.

When a user accesses a short URL, metrics are collected from the user and written asynchronously to TimescaleDB using a buffered channel pattern for efficient batch inserts.

The [Analytics API](./furi_analytics/) provides the frontend with the user metrics collected when a short URL is accessed. TimescaleDB continuous aggregates automatically maintain pre-computed metrics at 1-minute, 1-hour, and 1-day intervals for fast querying.

The project uses a simple architecture that can be scaled horizontally easily. To scale the process of accessing and shortening URLs, just instantiate more instances of the Shortener API and add them to the load balancer.

More details about each layer of the project can be found in the respective directory.

## 🔬 Tecnologies

The technologies that this project uses or has been written in are:

- [Shortener API](./furi/): [Rust](https://www.rust-lang.org/)
- [Analytics API](./furi_analytics/): [Rust](https://www.rust-lang.org/)
- [FrontEnd](./furi_web/): [Vue.js](https://vuejs.org/)
- [Web Server](./web-server/): [Jequi](https://github.com/Termack/jequi)
- Cache Database: [Redis](https://redis.io/)
- URLs Database: [Postgres](https://www.postgresql.org/)
- Analytics Database: [TimescaleDB](https://www.timescale.com/)
- CDN: [Cloudflare](https://www.cloudflare.com)

## 🏃‍♂️ Running Local

Clone the repository.
```sh
git clone git@github.com:Mewbi/furi.git
```

Set every required configuration and environment file. (Remember to edit the file content)
```sh
cp .env-example .env # Database config
cp ./furi/config-example.toml ./furi/config.toml # Furi API config
cp ./furi_analytics/config-example.toml ./furi_analytics/config.toml # Analytics API config
cp ./furi_web/.env-example ./furi_web/.env # Frontend config
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
127.0.0.1 redis postgres timescaledb furi.live www.furi.live
```

### 🛠️ Useful Tools

To access `postgres` (URLs database), execute

```sh
docker exec -it furi_postgres psql -U <user> -d <database>
```

To access `timescaledb` (Analytics database), execute

```sh
docker exec -it furi_timescaledb psql -U <user> -d analytics
```

Some useful TimescaleDB queries:

```sql
-- View raw analytics data
SELECT * FROM analytics_raw ORDER BY time DESC LIMIT 10;

-- View 1-minute aggregated metrics
SELECT * FROM analytics_1min WHERE uri = '<short_uri>' ORDER BY bucket DESC;

-- View continuous aggregate policies
SELECT * FROM timescaledb_information.jobs WHERE application_name LIKE '%aggregate%';
```

## 🤝 Contributing

Furi is an open-source project, so contributions are welcome!

Feel free to share suggestions for new features, improvements, and bug fixes.
