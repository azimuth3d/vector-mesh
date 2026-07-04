# Enterprise RAG Platform

A high-performance, multitenant chatbot system built with Rust, designed for online shops. It leverages a Vector Mesh architecture with GitOps and Monorepo practices to manage microservices, infrastructure, and data pipelines.

## 🏗️ Architecture Overview

The system follows a **Vector Mesh** pattern, decoupling ingestion, embedding, and querying into independent microservices orchestrated via Kubernetes and Argo CD.

- **apps/**: Source code for Rust microservices.
- **k8s/**: Kubernetes manifests managed by Argo CD.
- **Data Flow**:
  1. `ingestion-service` receives text/PDFs and pushes them to a Redis Queue.
  2. `embedding-service` consumes the queue, chunks text, generates embeddings via an external model, and stores vectors in Qdrant.
  3. `query-service` handles user questions, retrieves relevant context from Qdrant, and forwards it to Ollama for generation.

## 📦 Project Structure

```text
enterprise-rag-platform/
├── apps/                        # Source Code of Microservices (Rust)
│   ├── shared-lib/              # Shared library (OTel, Error Handling, DB Clients)
│   ├── ingestion-service/       # Ingests Text/PDF -> Redis Queue
│   ├── embedding-service/       # Queue -> Chunking -> Qdrant
│   └── query-service/           # Query -> Qdrant -> Ollama
├── k8s/                         # Kubernetes Manifests (Argo CD)
│   ├── base/                    # CRDs, Namespaces
│   ├── infrastructure/          # APISIX, Istio, Cert-manager, OTel, Jaeger
│   ├── databases/               # ScyllaDB, Qdrant, Redis
│   └── services/                # Deployments, Services, VirtualServices
└── .github/workflows/           # CI/CD Pipelines (Build Docker -> Update K8s)
```

## 🚀 Usage Instructions

### Prerequisites
- Rust 2024 Toolchain
- Docker & Docker Compose (for local testing)
- Kubernetes Cluster (v1.28+)
- Argo CD
- `kubectl` & `helm`

### Local Development
1. Clone the repository.
2. Build the shared library and services:
   ```bash
   cd apps/shared-lib && cargo build
   cd ../ingestion-service && cargo build
   cd ../embedding-service && cargo build
   cd ../query-service && cargo build
   ```
3. Run local dependencies (Redis, Qdrant, ScyllaDB) using Docker Compose or `kubectl apply -f k8s/databases/`.

### Kubernetes Deployment
1. Apply base infrastructure:
   ```bash
   kubectl apply -f k8s/base/
   kubectl apply -f k8s/infrastructure/
   ```
2. Deploy databases:
   ```bash
   kubectl apply -f k8s/databases/
   ```
3. Sync with Argo CD or apply service manifests:
   ```bash
   kubectl apply -f k8s/services/
   ```

## 🛠️ How To

- **Adding a new microservice**: Create a new directory under `apps/`, add a `Cargo.toml`, and replicate the structure of `shared-lib`. Update `k8s/services/` with the new Deployment and Service manifests.
- **Configuration**: Environment variables are injected via Kubernetes Secrets/ConfigMaps. Update `k8s/services/` or `k8s/infrastructure/` accordingly.
- **Observability**: OpenTelemetry is initialized in `shared-lib/src/otel.rs`. Metrics and traces are exported to the configured OTel Collector.

## 🔍 Areas for Improvement

- **Error Handling**: Ensure all services strictly propagate errors using `?` and `AppError` without `.unwrap()` or `.expect()`.
- **Testing**: Add unit and integration tests for `shared-lib` and each service.
- **Performance**: Optimize chunking strategies in `embedding-service` and implement connection pooling for Qdrant/ScyllaDB.
- **Security**: Implement mTLS via Istio, enforce RBAC, and rotate secrets regularly.
- **CI/CD**: Automate Docker image builds, security scanning, and K8s manifest updates in `.github/workflows/`.
- **Documentation**: Expand inline code comments and add API documentation using `cargo doc`.

## 📜 License

[Add License Here]
