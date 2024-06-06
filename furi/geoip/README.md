# 🌎 GeoIP 🌎

This project uses the [MaxMind](https://www.maxmind.com/en/home) geoip database.

You can get the free database from their site. It is required to [create an account](https://support.maxmind.com/hc/en-us/articles/4407099783707-Create-an-Account#h_01G4G4NG5C63BQ6HRG6MSS50T3) to access the free GeoIP database.

The project expects two different databases: the `country` and `city` scopes.

Save them in this directory with the following names:
- GeoLite2-Country.mmdb
- GeoLite2-City.mmdb

Alternatively, you can change the filepath in the Furi configuration file.