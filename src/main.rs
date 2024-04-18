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
mod container;
mod domain;
mod infrastructure;

pub mod auth {
    tonic::include_proto!("auth");
}

pub mod team {
    tonic::include_proto!("team");
}

pub mod task {
    tonic::include_proto!("task");
}

mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    prometheus_exporter::init();

    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("debug")) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    let addr = "0.0.0.0:50051".parse()?;

    let container = Container::default();

    tokio::spawn(async move {
        tracing::info!(message = "Starting server 🙂", %addr);
        Server::builder()
            // .tls_config(tls)
            // .expect("Failed to add tls config")
            .trace_fn(|_| tracing::debug_span!("taskem"))
            .layer(container.layer)
            .http2_keepalive_interval(Some(Duration::from_secs(3)))
            .add_service(container.auth_server)
            .add_service(container.team_server)
            .add_service(container.task_server)
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
