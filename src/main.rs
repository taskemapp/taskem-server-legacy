use crate::container::Container;




use dotenv::dotenv;
use std::env;
use std::time::Duration;


use tonic::{
    transport::{Server},
};




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

    // let cert = tokio::fs::read("certs/server.pem")
    //     .await
    //     .expect("Failed to read cert");
    // let key = tokio::fs::read("certs/server.key")
    //     .await
    //     .expect("Failed to read key");

    // let identity = Identity::from_pem(cert, key);

    // let tls = tonic::transport::ServerTlsConfig::new().identity(identity);

    let addr = "0.0.0.0:50051".parse()?;

    let container = Container::default();

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

    Ok(())
}
