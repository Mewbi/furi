# 🔒 SSL Certificates 🔒

The web server expects to use SSL certificates to run.

The expected filenames are `furi-cert.pem` and `furi-key.key`. Alternatively, you can change the [docker-compose](../docker-compose.yml) file to specify different certificate files.

This project uses a CDN layer before the web server, so technically the SSL could be a self-signed one. You have three different ways to generate the SSL files:

## 1 - Using Certbot 🤖

This is the best free way to generate SSL certificates because you can generate a valid wildcard SSL, and you will not encounter any browser issues related to accessing insecure content.

This method requires a domain and a server with a public IP (e.g., VPS).

The instructions to generate certificates depend on the system used. You can check the tutorial [here](https://certbot.eff.org/instructions)

## 2 - Using Cloudflare SSL ☁️

This is the easiest method and requires a domain.

After adding the domain in Cloudflare, generate an origin certificate and save the files in this directory.

These certificates are valid only when accessed through Cloudflare, so you may encounter issues running locally related to accessing content with invalid SSL certificates.

This [tutorial](https://developers.cloudflare.com/ssl/origin-configuration/origin-ca/) explain the process with more details.

## 3 - Using Self Signed SSL 🖊️

This method doesn't require anything other than the tools available on most systems.

Create a file `openssl.cnf` with content
```
[ req ]
default_bits       = 2048
prompt             = no
default_md         = sha256
req_extensions     = req_ext
distinguished_name = dn

[ dn ]
C  = US
ST = California
L  = San Francisco
O  = Your Company
OU = Your Unit
CN = furi.live

[ req_ext ]
subjectAltName = @alt_names

[ alt_names ]
DNS.1 = furi.live
DNS.2 = www.furi.live

[ v3_ca ]
subjectAltName = @alt_names
```

Remember to change the `alt_names` with the domains names that you want. Then generate the `crt` and `key` files

```sh
openssl req -x509 -sha256 -nodes -days 3650 -newkey rsa:2048 -keyout furi-key.key -out furi-cert.crt  -config openssl.cnf
```

To convert `crt` to `pem` file
```sh
openssl x509 -in furi-cert.crt -out furi-cert.pem -outform PEM
``` 