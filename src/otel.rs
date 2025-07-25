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

        let addr = SocketAddr::from(([0, 0, 0, 0], 9464));
        let listener = TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}

// pub fn init_tracer() {
//     let registry = prometheus::Registry::new();
//     // Создаем экспортер для вывода метрик в stdout (консоль)
//     let exporter = opentelemetry_prometheus::exporter()
//         .with_registry(registry.clone())
//         .build()
//         .unwrap();

//     // Создаем провайдер метрик с периодическим чтением (исправленная строка)
//     let provider = SdkMeterProvider::builder().with_reader(exporter).build();
//     let meter = provider.meter("my-app");

//     // Получаем метр для нашего сервиса

//     // Создаем счетчик
//     let counter = meter.u64_counter("request.count").build();

//     // Записываем измерение
//     counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);

//     // Создаем ObservableCounter с callback-функцией
//     let _observable_counter = meter
//         .u64_observable_counter("bytes_received")
//         .with_callback(|observer| observer.observe(100, &[KeyValue::new("protocol", "udp")]))
//         .build();
// }

// pub async fn new_telemetry() {
//     let registry = prometheus::Registry::new();
//     let exporter = opentelemetry_prometheus::exporter()
//         .with_registry(registry.clone())
//         .build()
//         .unwrap();

//     let provider = SdkMeterProvider::builder().with_reader(exporter).build();
//     let meter = provider.meter("my-app");

//     // 1. Основные счётчики
//     let request_counter = meter
//         .u64_counter("http.requests.total")
//         .with_description("Total HTTP requests")
//         .build();

//     let error_counter = meter
//         .u64_counter("http.errors.total")
//         .with_description("Total HTTP errors")
//         .build();

//     // 2. Гистограммы для времени выполнения
//     let latency_histogram = meter
//         .f64_histogram("http.request.duration.seconds")
//         .with_description("HTTP request duration in seconds")
//         .build();

//     // 3. Gauge для текущего состояния
//     let active_connections = meter
//         .i64_gauge("db.connections.active")
//         .with_description("Active database connections")
//         .build();

//     // Пример использования
//     request_counter.add(
//         1,
//         &[
//             KeyValue::new("endpoint", "/api/users"),
//             KeyValue::new("method", "GET"),
//         ],
//     );
//     error_counter.add(
//         1,
//         &[
//             KeyValue::new("endpoint", "/api/users"),
//             KeyValue::new("method", "GET"),
//         ],
//     );
//     latency_histogram.record(0.5, &[KeyValue::new("endpoint", "/api/users")]);
//     active_connections.record(1, &[]);

//     // Метрики для бизнес-логики
//     let orders_processed = meter
//         .u64_counter("orders.processed.total")
//         .with_description("Total processed orders")
//         .build();

//     orders_processed.add(1, &[KeyValue::new("type", "premium")]);

//     run_metrics_server(registry).await
// }

// pub async fn run_metrics_server(registry: Registry) {
//     let app = Router::new().route(
//         "/metrics",
//         get(move || {
//             let registry = registry.clone();
//             async move {
//                 let encoder = TextEncoder::new();
//                 let metric_families = registry.gather();
//                 let mut buffer = vec![];
//                 encoder.encode(&metric_families, &mut buffer).unwrap();

//                 // Создаем headers с правильным Content-Type
//                 let mut headers = HeaderMap::new();
//                 headers.insert(
//                     "Content-Type",
//                     HeaderValue::from_static("text/plain; version=0.0.4"),
//                 );

//                 Response::builder()
//                     .header("Content-Type", "text/plain; version=0.0.4")
//                     .body(Body::from(buffer))
//                     .unwrap()
//             }
//         }),
//     );

//     let addr = SocketAddr::from(([0, 0, 0, 0], 9464));
//     let listener = TcpListener::bind(addr).await.unwrap();

//     axum::serve(listener, app.into_make_service())
//         .await
//         .unwrap();
// }
