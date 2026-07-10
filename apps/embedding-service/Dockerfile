# syntax=docker/dockerfile:1
FROM rust:1.89-slim
WORKDIR /app
COPY target/release/embedding-service /app/embedding-service

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -r -g appuser appuser

COPY --from=builder /app/embedding-service /usr/local/bin/embedding-service

USER appuser

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/embedding-service"]
