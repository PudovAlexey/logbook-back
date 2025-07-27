use axum::{body::Body, response::Response, routing::get, Router};
use http::{HeaderMap, HeaderValue};
use opentelemetry::{
    global,
    metrics::{Counter, Meter, MeterProvider as _},
    KeyValue,
};
use opentelemetry_prometheus::{self};
use opentelemetry_sdk::metrics::SdkMeterProvider;
use prometheus::{Encoder, Registry, TextEncoder};
use std::{net::SocketAddr, sync::Arc};

use tokio::net::TcpListener;

use crate::common::env::ENV;

pub struct Metrics {
    pub request_counter: Counter<u64>,
}

pub struct MetricsSubscriber {
    pub registry: Arc<Registry>,
    pub metrics: Arc<Metrics>,
    // pub exporter: PrometheusExporter,
}

impl MetricsSubscriber {
    pub fn new() -> Self {
        let registry = Registry::new();

        // Создаем экспортер
        let exporter = opentelemetry_prometheus::exporter()
            .with_registry(registry.clone())
            .build()
            .unwrap();


        let provider = SdkMeterProvider::builder().with_reader(exporter).build();

        global::set_meter_provider(provider);


        let meter = global::meter("my-app");

        let request_counter = meter
            .u64_counter("http.requests.total")
            .with_description("Total HTTP requests")
            .build();


        Self {
            registry: Arc::new(registry),
            metrics: Arc::new(Metrics { request_counter }),
        }
    }

    pub async fn run_metrics_server(&mut self) {
        let registry = self.registry.clone();

        let app = Router::new().route(
            "/metrics",
            get(move || {
                let registry = registry.clone();
                async move {
                    let encoder = TextEncoder::new();
                    let metric_families = registry.gather();
                    let mut buffer = vec![];
                    encoder.encode(&metric_families, &mut buffer).unwrap();

                    // Создаем headers с правильным Content-Type
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        "Content-Type",
                        HeaderValue::from_static("text/plain; version=0.0.4"),
                    );

                    Response::builder()
                        .header("Content-Type", "text/plain; version=0.0.4")
                        .body(Body::from(buffer))
                        .unwrap()
                }
            }),
        );

            let api_host = ENV::new().app_host;
            let metrics_port = ENV::new().metrics_port;

            let addr = SocketAddr::from((api_host, metrics_port));

                println!("the metrics server listening on http://{:?}/metrics", addr);

        let listener = TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}