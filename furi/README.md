# 🦀 Shortener API

The responsibility of this API is to create a short URL for a given URL and redirect to the original URL when a short URL is received.

## ⚙️ API Endpoints

The API accepts only JSON bodies.

### GET /:id

Redirects to the original URL.

| Name | Path | Type | Description
|-------------:|:--------:|:-------:| --- |
| `id` | URL | string  | Generated short URI. By default it's a string with size 7. |

### POST /url

Creates a short URL that redirects to the original URL.

| Name | Path | Type | Description
|-------------:|:--------:|:-------:| --- |
| `url` | body | string  | Original URL. |

#### Response

```json
{
    "original_url": "https://domain.com/my-long-url",
    "short_url": "https://furi.live/abcdefg"
}
```

## 🏃‍♂️ Running Local

It is possible to run only the shortener API locally.

First of all, ensure you have Rust installed.

Then, set the configuration file based on the configuration example file. (Remember to change values if required)

```sh
cp ./config-example.toml config.toml
```

Set the GeoIP database as explained [here](./geoip/).

Start the message broker and all databases. This can be done using Docker Compose or manually.

Finally, start the API using:

```sh
cargo run
```