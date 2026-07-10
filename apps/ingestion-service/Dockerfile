# syntax=docker/dockerfile:1
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -r -g appuser appuser

COPY target/release/ingestion-service /app/ingestion-service

USER appuser

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/ingestion-service"]
