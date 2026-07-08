# syntax=docker/dockerfile:1
FROM rust:1.89-slim AS builder

WORKDIR /app
ENV CARGO_NET_RETRY=3

RUN apt-get update && apt-get install -y --no-install-recommends build-essential pkg-config libssl-dev clang libsqlite3-dev && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

RUN strip target/release/embedding-service

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -r -g appuser appuser

COPY --from=builder /app/target/release/embedding-service /usr/local/bin/embedding-service

USER appuser

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/embedding-service"]
