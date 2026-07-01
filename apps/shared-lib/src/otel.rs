use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::TracerProvider as SdkTracerProvider;
use tracing::info;
use crate::error::{AppError, Result};

pub fn init_tracer() -> Result<()> {
    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .build()
                .map_err(|e| AppError::Internal(e.to_string()))?,
            Tokio,
        )
        .build();

    global::set_tracer_provider(tracer_provider.clone());

    let _tracer = tracer_provider.tracer("enterprise-rag-platform");
    info!("OpenTelemetry tracer initialized");

    Ok(())
}
