Phase 1: Infrastructure & GitOps Foundation
Goal: Provision the Kubernetes Cluster (supporting Spot/On-Demand nodes), install the Service Mesh, Gateway, and GitOps workflows.

Task 1.1: Kubernetes & Istio Setup
[x] Write Kubernetes Manifests (Kustomize) to install Istio Service Mesh with Strict mTLS across the entire rag-platform namespace. Include Node Affinity configurations to separate workloads: deploy StatefulSets on On-Demand nodes, and Stateless/AI Inference workloads on Spot instances.

Task 1.2: APISIX & Cert-manager Setup
[x] Write Helm values/Manifests to install Apache APISIX as the primary Ingress Gateway and deploy cert-manager for SSL management. In the cert-manager solver configuration, ensure the use of class instead of ingressClassName to comply with the latest Kubernetes standards.

Task 1.3: Argo CD Bootstrap
[x] Write Argo CD Application manifests utilizing the "App of Apps" pattern. Configure Argo CD to point to the k8s/infrastructure/, k8s/databases/, and k8s/services/ directories within the Git repository, and enable Auto-sync.

Phase 2: High-Performance Data & AI Layer
Goal: Deploy all databases in a distributed architecture and configure Local LLMs.

Task 2.1: ScyllaDB & Redis Deployment

Write Kubernetes StatefulSet and Headless Service manifests for a ScyllaDB cluster (3 nodes) and a Redis instance (for Queue/Cache). Additionally, design a highly optimized ScyllaDB CQL Schema for storing chat history. The schema must be designed without joins, prioritizing maximum Read/Write performance.

Task 2.2: Qdrant & Ollama Deployment

Write Kubernetes Deployment manifests for Qdrant (Vector Database) and Ollama (including the LLM model). Ensure the Ollama Pod has appropriate resource requests/limits and utilizes a Node Selector to force deployment onto GPU-enabled Spot Nodes.

Phase 3: Rust Microservices (Core Logic)
Goal: Develop the backend using Rust and Axum, strictly adhering to Clean Architecture principles.

Task 3.1: Shared Library & Observability Setup

Create a Rust Library crate (shared-lib) containing the following core modules: 1. OpenTelemetry setup (OTLP Exporter), 2. Custom Error Handling for Axum, and 3. Database connection pools for ScyllaDB and Redis. Structure the code adhering to Clean Architecture principles.

Task 3.2: Ingestion & Embedding Service

Write a Rust Microservice (embedding-service) that acts as a Background Worker. It must pull data from the Redis Queue, perform text chunking, send API requests to an Embedding Model, and upsert the resulting vectors into Qdrant. All core logic must be wrapped with OpenTelemetry Spans (#[tracing::instrument]) for Distributed Tracing.

Task 3.3: Query Service (RAG Logic)

Write a Rust Axum API (query-service) that accepts HTTP POST requests. It should query Qdrant for the most relevant context, combine the context with Prompt Engineering, send the payload to Ollama via API, save the chat history to ScyllaDB, and return the result as a JSON response. The folder structure must strictly follow Clean Architecture (Domain, Use Cases, Infrastructure, Adapters).

🔍 Phase 4: Observability & Service Mesh Routing
Goal: Achieve complete observability (Tracing) and manage traffic effectively via Istio.

Task 4.1: OpenTelemetry Collector & Jaeger

Write Kubernetes Manifests to deploy an OpenTelemetry Collector as a DaemonSet to receive data via OTLP gRPC. Configure it to export telemetry data to a Jaeger instance (All-in-one deployment) to visualize Distributed Tracing across all microservices.

Task 4.2: APISIX to Istio Routing

Write APISIX Route configurations and Istio VirtualService manifests. The system must accept external HTTPS traffic via APISIX and route it securely to the query-service and ingestion-service utilizing Istio's mTLS layer.

💡 Execution Plan:
You can set up the folder structure above and copy/paste the task descriptions directly to your AI coding assistant (like Aider) one by one. This will generate precise code and YAML files that strictly adhere to your architectural requirements, resulting in a production-ready Full-Stack portfolio project.
