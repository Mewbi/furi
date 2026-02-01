# 🦀 Analytcs API

The responsibility of this API is to show analytics access data from shortened URLs.

## ⚙️ API Endpoints

The API accepts only JSON bodies.

### POST /:id

Get analytics data of shortened URL.

| Name | Path | Type | Description
|-------------:|:--------:|:-------:| --- |
| `id` | URL | string  | Generated short URI. By default it's a string with size 7. |
| `aggregation` | Body | string  | Specify how the data is aggregated. Accepts `1min`, `1hour`, `1day`. |
| `from` | Body | int  | Start date, in timestamp, of metrics. |
| `to` | Body | int  | End date, in timestamp, of metrics. |

#### Response

```json
{
  "uri": "12jkn3a",
  "req_total": 31,
  "req_time_series": [
    {
      "date": 1718750820,
      "count": 21
    },
    {
      "date": 1718750880,
      "count": 10
    }
  ],
  "device_count": [
    {
      "device_type": "Android",
      "count": 6
    },
    {
      "device_type": "Iphone",
      "count": 7
    },
    {
      "device_type": "Linux",
      "count": 4
    },
    {
      "device_type": "Mac",
      "count": 2
    },
    {
      "device_type": "Windows",
      "count": 8
    }
  ],
  "country_access": [
    {
      "country": "AU",
      "count": 5
    },
    {
      "country": "BR",
      "count": 9
    },
    {
      "country": "CA",
      "count": 5
    },
    {
      "country": "CN",
      "count": 8
    },
    {
      "country": "DE",
      "count": 4
    }
  ]
}
```

## 🏃‍♂️ Running Local

It is possible to run only the analytics API locally.

First of all, ensure you have Rust installed.

Then, set the configuration file based on the configuration example file. (Remember to change values if required)

```bash
cp ./config-example.toml config.toml
```

Start the message broker and all databases. This can be done using Docker Compose or manually.

Finally, start the API using:

```bash
cargo run
```
