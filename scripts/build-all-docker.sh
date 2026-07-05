#!/bin/bash
set -e

echo "🚀 Building all Docker images..."

SERVICES=("ingestion-service" "embedding-service" "query-service")

for SERVICE in "${SERVICES[@]}"; do
    echo "📦 Building $SERVICE..."
    docker build -t "$SERVICE" "./apps/$SERVICE"
    echo "✅ $SERVICE built successfully."
done

echo "🎉 All Docker images built successfully."
