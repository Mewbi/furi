FROM rust:1.78 AS builder

RUN apt-get update && apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new --bin furi

WORKDIR /app

COPY Cargo.toml Cargo.lock config.toml ./
COPY config.prod.toml ./config.toml

COPY src ./src

COPY geoip ./geoip

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

RUN apk --no-cache add ca-certificates libgcc gcompat

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/furi .
COPY --from=builder /app/config.toml .
COPY --from=builder /app/src/infrastructure/migrations/ ./src/infrastructure/migrations/
COPY --from=builder /app/geoip/ ./geoip/

EXPOSE 3000

CMD ["./furi"]
