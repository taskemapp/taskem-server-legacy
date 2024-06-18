use std::{net::SocketAddr, time::Duration};

use autometrics::prometheus_exporter;
use axum::{routing::get, Extension, Router};
use dotenv::{dotenv, from_filename};
use hyper::StatusCode;
use tokio::net::TcpListener;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tower::ServiceBuilder;

use crate::{api::services::file::user_file_handler, app, container::Container};

pub async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "e2e"))]
    dotenv().ok();

    #[cfg(feature = "e2e")]
    from_filename(".env.test").ok();

    prometheus_exporter::init();

    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let grpc_addr = "0.0.0.0:50051";
    let file_addr = "0.0.0.0:9090";
    let metrics_addr = "0.0.0.0:3000";

    let container = Container::new().await;

    start_grpc_server(container.clone(), grpc_addr, false).await?;
    start_file_server(container, file_addr).await?;
    start_metrics_server(metrics_addr).await?;

    Ok(())
}

pub async fn start_grpc_server(
    container: Container,
    addr: &str,
    tls: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse().unwrap();
    tokio::spawn(async move {
        tracing::info!(message = "Starting server 🙂", %addr);
        if tls {
            let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "tls"]);
            let cert = std::fs::read_to_string(data_dir.join("tls/server.pem")).unwrap();
            let key = std::fs::read_to_string(data_dir.join("tls/server.key")).unwrap();

            let identity = Identity::from_pem(cert, key);

            let tls_conf = ServerTlsConfig::new().identity(identity);

            Server::builder()
                .tls_config(tls_conf)
                .expect("Failed to add tls config")
                .trace_fn(|_| tracing::debug_span!("taskem"))
                .layer(container.layer)
                .http2_keepalive_interval(Some(Duration::from_secs(3)))
                .add_service(container.auth_server)
                .add_service(container.team_server)
                .add_service(container.task_server)
                .add_service(container.profile_server)
                .serve(addr)
                .await
                .expect("Serve failed");
        } else {
            Server::builder()
                .trace_fn(|_| tracing::debug_span!("taskem"))
                .layer(container.layer)
                .http2_keepalive_interval(Some(Duration::from_secs(3)))
                .add_service(container.auth_server)
                .add_service(container.team_server)
                .add_service(container.task_server)
                .add_service(container.profile_server)
                .serve(addr)
                .await
                .expect("Serve failed");
        }
    })
    .await?;

    Ok(())
}

pub async fn start_metrics_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = addr.parse().unwrap();
    tokio::spawn(async move {
        let app = Router::new().route(
            "/metrics",
            get(|| async {
                prometheus_exporter::encode_to_string()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            }),
        );

        tracing::info!(message = "Starting axum server for metrics", %addr);
        let tcp_listen = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(tcp_listen, app.into_make_service())
            .await
            .unwrap();
    })
    .await?;

    Ok(())
}

pub async fn start_file_server(
    container: Container,
    addr: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = addr.parse().unwrap();
    tokio::spawn(async move {
        let app = Router::new()
            .nest(
                "/file",
                Router::new().route("/users/:user_name/:file_name", get(user_file_handler)),
            )
            .layer(Extension(container.file_service_data))
            .layer(ServiceBuilder::new().layer(container.auth_layer));

        tracing::info!(message = "Starting axum server for file 📁", %addr);
        let tcp_listen = TcpListener::bind(addr).await.unwrap();
        axum::serve(tcp_listen, app.into_make_service())
            .await
            .unwrap();
    })
    .await?;

    Ok(())
}
