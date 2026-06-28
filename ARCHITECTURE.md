Vector Mesh
โครงสร้าง Repository (GitOps + Monorepo)
แบ่งเป็น 2 ส่วนหลักคือ apps/ (สำหรับ Source Code) และ k8s/ (สำหรับ Argo CD Manifests)

enterprise-rag-platform/
├── apps/                        # Source Code ของ Microservices (Rust)
│   ├── shared-lib/              # Library กลาง (OTel, Error Handling, DB Client)
│   ├── ingestion-service/       # รับ Text/PDF -> ยัดลง Redis Queue
│   ├── embedding-service/       # ดึง Queue -> ทำ Chunking -> เซฟลง Qdrant
│   └── query-service/           # รับคำถาม -> ค้นหา Qdrant -> ส่งให้ Ollama
├── k8s/                         # Kubernetes Manifests (สำหรับ Argo CD)
│   ├── base/                    # ค่าพื้นฐาน (CRDs, Namespaces)
│   ├── infrastructure/          # APISIX, Istio, Cert-manager, OTel, Jaeger
│   ├── databases/               # ScyllaDB, Qdrant, Redis
│   └── services/                # Deployment, Service, VirtualService ของแอปเรา
└── .github/workflows/           # CI/CD Pipelines (Build Docker -> Update K8s Manifests)
