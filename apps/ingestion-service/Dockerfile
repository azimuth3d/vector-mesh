# syntax=docker/dockerfile:1
FROM rust:1.89-slim AS builder
WORKDIR /app
COPY target/release/ingestion-service /app/ingestion-service

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -r -g appuser appuser

COPY --from=builder /app/ingestion-service /usr/local/bin/ingestion-service

USER appuser

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/ingestion-service"]
