use crate::api::services::file::user_file_handler;
use crate::container::Container;
use autometrics::prometheus_exporter;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};
use dotenv::dotenv;
use std::net::SocketAddr;
use std::time::Duration;
use tonic::transport::Server;
use tower::ServiceBuilder;

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

pub mod profile {
    tonic::include_proto!("profile");
}

mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    prometheus_exporter::init();

    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    let addr = "0.0.0.0:50051".parse()?;

    let container = Container::new().await;

    tokio::spawn(async move {
        tracing::info!(message = "Starting server 🙂", %addr);
        Server::builder()
            // .tls_config(tls)
            // .expect("Failed to add tls config")
            .trace_fn(|_| tracing::debug_span!("taskem"))
            .layer(container.layer.clone())
            .http2_keepalive_interval(Some(Duration::from_secs(3)))
            .add_service(container.auth_server)
            .add_service(container.team_server)
            .add_service(container.task_server)
            .add_service(container.profile_server)
            .serve(addr)
            .await
            .expect("Serve failed");
    });

    tokio::spawn(async move {
        let app = Router::new().route(
            "/metrics",
            get(|| async {
                prometheus_exporter::encode_to_string()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            }),
        );

        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

        tracing::info!(message = "Starting axum server for metrics", %addr);
        let tcp_listen = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(tcp_listen, app.into_make_service())
            .await
            .unwrap();
    });

    let app = Router::new()
        .route("/file", get(user_file_handler))
        .layer(Extension(container.file_service_data))
        .layer(ServiceBuilder::new().layer(container.auth_layer));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));

    tracing::info!(message = "Starting axum server for file 📁", %addr);
    let tcp_listen = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp_listen, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
