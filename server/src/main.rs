use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// const DATASET_NAME: &str = "sample-sample-sample";
// const API_KEY: &str = "MZBfvYGhzdsKPpOjeiei1M";

#[tokio::main]
async fn main() {
    // let otel_exporter_otlp_endpoint =
    //     env::var("OTEL_EXPORTER_OTLP_ENDPOINT").expect("OTEL_EXPORTER_OTLP_ENDPOINT must be set");
    // let otel_exporter_otlp_headers =
    //     env::var("OTEL_EXPORTER_OTLP_HEADERS").expect("OTEL_EXPORTER_OTLP_HEADERS must be set");
    // let otel_service_name = env::var("OTEL_SERVICE_NAME").expect("OTEL_SERVICE_NAME must be set");

    // let mut map = tonic::metadata::MetadataMap::with_capacity(1);
    // map.insert(
    //     "x-honeycomb-team",
    //     "MZBfvYGhzdsKPpOjeiei1M".parse().unwrap(),
    // );

    // let tracer = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     .with_exporter(
    //         opentelemetry_otlp::new_exporter()
    //             .http()
    //             .with_endpoint("https://api.honeycomb.io/v1/traces")
    //             .with_http_client(reqwest::Client::new())
    //             .with_headers(HashMap::from([
    //                 ("x-honeycomb-dataset".into(), DATASET_NAME.into()),
    //                 ("x-honeycomb-team".into(), API_KEY.into()),
    //             ]))
    //             .with_timeout(std::time::Duration::from_secs(2)),
    //     )
    //     .install_batch(opentelemetry_sdk::runtime::Tokio)
    //     .unwrap();

    // let otel_layer = OpenTelemetryLayer::new(tracer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        // .with(otel_layer)
        .init();

    // build our application with a route
    let app = Router::new().route("/", get(handler)).layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true)),
    );

    // run it
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
