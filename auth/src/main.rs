use crate::container::Container;
use autometrics::prometheus_exporter;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tonic::transport::Server;

mod api;
mod constants;
mod container;
mod domain;
mod infrastructure;

mod core;

pub mod auth {
    tonic::include_proto!("auth");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    prometheus_exporter::init();

    let _args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = "0.0.0.0:50001".parse()?;

    let container = Container::default();

    tokio::spawn(async move {
        tracing::info!(message = "Starting auth service 🙂", %addr);
        Server::builder()
            // .tls_config(tls)
            // .expect("Failed to add tls config")
            .trace_fn(|_| tracing::debug_span!("taskem"))
            .layer(container.layer)
            .http2_keepalive_interval(Some(Duration::from_secs(3)))
            .add_service(container.auth_server)
            .serve(addr)
            .await
            .expect("Serve failed");
    });

    let app = Router::new().route(
        "/metrics",
        get(|| async {
            prometheus_exporter::encode_to_string().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        }),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!(message = "Starting axum server for metrics", %addr);

    let tcp_listen = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp_listen, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
